use aws_sdk_dynamodb::{
    error::GetItemErrorKind::ResourceNotFoundException, model::AttributeValue, Client,
    SdkError::ServiceError,
};
use chrono::Utc;
use codenames_domain::{
    game::{dao::GameDao, model::GameData},
    DaoError,
    DaoError::NotFound,
    DaoResult, Lowercase, StdResult,
};
use redis::{Commands, Connection, Value};

pub struct DynamoDao {
    client: Client,
}

impl DynamoDao {
    pub fn new() -> StdResult<DynamoDao> {
        let shared_config = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(aws_config::load_from_env());
        let client = Client::new(&shared_config);
        Ok(DynamoDao { client })
    }
}

impl Clone for DynamoDao {
    fn clone(&self) -> Self {
        DynamoDao::new().unwrap()
    }
}

impl GameDao for DynamoDao {
    fn get(&mut self, key: Lowercase) -> DaoResult<GameData> {
        let request = &self
            .client
            .get_item()
            .table_name("codenames")
            .key("id", AttributeValue::S(key.value().to_string()));

        let result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(request.clone().send())
            .map_err(|outer| match outer {
                ServiceError { err, .. } => match &err.kind {
                    ResourceNotFoundException(r) => DaoError::NotFound(r.to_string()),
                    _ => DaoError::Unknown("wat".into()),
                },
                e => DaoError::Unknown(e.to_string()),
            })?;

        let item = result.item.ok_or(NotFound(key.value().to_string()))?;
        let attribute = item.get("game").ok_or(DaoError::Unknown(
            "could not find game attribute on dynamo result".into(),
        ))?;
        let game_string = attribute
            .as_s()
            .map_err(|_| DaoError::Unknown("could not get game as string".into()))?;

        serde_json::from_str(game_string).map_err(|e| DaoError::Unknown(e.to_string()))
    }

    fn keys(&mut self) -> DaoResult<Vec<Lowercase>> {
        todo!()
        // let request = &self.client
        //     .scan()
        //     .table_name("codenames")
        //     .attributes_to_get(vec!["id"]);
        //
        // let result = request.send().await?;
        // let items = result.items.unwrap_or(Vec::new());
        // let keys: Vec<String> = items.iter().map(|i| i.get("id")).collect();
        // return Ok(keys.iter().map(|k| Lowercase::new(k)).collect())
    }

    fn set(&mut self, key: Lowercase, game: GameData) -> DaoResult<()> {
        let request = &self
            .client
            .put_item()
            .table_name("codenames")
            .item("id", AttributeValue::S(key.value().to_string()))
            .item("timestamp", AttributeValue::S(Utc::now().to_string()))
            .item("game", AttributeValue::S(json!(game).to_string()));

        match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(request.clone().send())
        {
            Ok(_) => Ok(()),
            Err(e) => Err(DaoError::Unknown(e.to_string())),
        }
    }
}

pub struct RedisDao {
    con: Connection,
}

impl Clone for RedisDao {
    fn clone(&self) -> Self {
        RedisDao::new().unwrap()
    }
}

impl RedisDao {
    pub fn new() -> StdResult<RedisDao> {
        let host = std::env::var("REDIS_HOST")?;
        let client = redis::Client::open(format!("redis://{}/", host))?;
        let con = client.get_connection()?;
        Ok(RedisDao { con })
    }
}

impl GameDao for RedisDao {
    fn get(&mut self, key: Lowercase) -> DaoResult<GameData> {
        let value: Value = self
            .con
            .get(key.value().clone())
            .map_err(|e| DaoError::Unknown(e.to_string()))?;

        let result: String = match value {
            Value::Data(bytes) => {
                std::str::from_utf8(&bytes)
                    .map(|s| s.to_string())
                    .map_err(|_| {
                        DaoError::Unknown(
                            "could not parse redis::Value::Data to string".to_string(),
                        )
                    })
            }
            Value::Nil => Err(DaoError::NotFound(key.value().to_string())),
            _ => Err(DaoError::Unknown(
                "unexpected redis::Value type from get operation".to_string(),
            )),
        }?;

        serde_json::from_str(result.as_str()).map_err(|e| DaoError::Unknown(e.to_string()))
    }

    fn keys(&mut self) -> DaoResult<Vec<Lowercase>> {
        let result: Vec<Lowercase> = self
            .con
            .keys("*")
            .map(|ks: Vec<String>| ks.iter().map(|k| Lowercase::new(k)).collect())
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        Ok(result)
    }

    fn set(&mut self, key: Lowercase, game: GameData) -> DaoResult<()> {
        self.con
            .set_ex(key.value(), json!(game).to_string(), 86400)
            .map_err(|e| DaoError::Unknown(e.to_string()))
    }
}
