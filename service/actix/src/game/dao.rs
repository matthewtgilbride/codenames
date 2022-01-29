use aws_sdk_dynamodb::{
    error::GetItemErrorKind::ResourceNotFoundException, model::AttributeValue, Client,
    SdkError::ServiceError,
};
use chrono::{Duration, Utc};
use codenames_domain::{
    game::{dao::GameDao, model::GameData},
    DaoError,
    DaoError::NotFound,
    DaoResult, Lowercase, StdResult,
};
use redis::{Commands, Connection, Value};
use tokio::runtime::Runtime;

const DYNAMO_TABLE_NAME: &str = "codenames";
const DYNAMO_KEY_ATTRIBUTE: &str = "key";
const DYNAMO_TTL_ATTRIBUTE: &str = "ttl";
const DYNAMO_GAME_ATTRIBUTE: &str = "game";

pub struct DynamoDao {
    client: Client,
    runtime: Runtime,
}

impl DynamoDao {
    pub fn new() -> StdResult<DynamoDao> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let shared_config = runtime.block_on(aws_config::load_from_env());
        let client = Client::new(&shared_config);
        Ok(DynamoDao { client, runtime })
    }

    fn get_ttl() -> i64 {
        Utc::now().timestamp() + Duration::days(1).num_seconds()
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
            .table_name(DYNAMO_TABLE_NAME)
            .key(DYNAMO_KEY_ATTRIBUTE, AttributeValue::S(key.value().to_string()));

        let result =
            self.runtime
                .block_on(request.clone().send())
                .map_err(|outer| match outer {
                    ServiceError { err, .. } => match &err.kind {
                        ResourceNotFoundException(r) => DaoError::NotFound(r.to_string()),
                        _ => DaoError::Unknown("unknown dynamo service error".into()),
                    },
                    e => DaoError::Unknown("unknown dynamo sdk error".into()),
                })?;

        let item = result.item.ok_or(NotFound(key.value().to_string()))?;
        let attribute = item.get(DYNAMO_GAME_ATTRIBUTE).ok_or(DaoError::Unknown(
            "could not find game attribute on dynamo result".into(),
        ))?;
        let game_string = attribute
            .as_s()
            .map_err(|_| DaoError::Unknown("could not get game as string".into()))?;

        serde_json::from_str(game_string).map_err(|e| DaoError::Unknown(e.to_string()))
    }

    fn keys(&mut self) -> DaoResult<Vec<Lowercase>> {
        let request = &self
            .client
            .scan()
            .table_name(DYNAMO_TABLE_NAME)
            .attributes_to_get(DYNAMO_KEY_ATTRIBUTE);

        let result = self
            .runtime
            .block_on(request.clone().send())
            .map_err(|e| DaoError::Unknown(e.to_string()))?;

        let items = result
            .items
            .ok_or_else(|| DaoError::Unknown("no items".into()))?;

        let keys: Vec<String> = items
            .iter()
            .map(|i| i.get(DYNAMO_KEY_ATTRIBUTE).expect(format!("No {} field in response", DYNAMO_KEY_ATTRIBUTE).as_str()))
            .map(|a| a.as_s().expect(format!("{} field was not a string", DYNAMO_KEY_ATTRIBUTE).as_str()))
            .cloned()
            .collect();

        Ok(keys.iter().map(|k| Lowercase::new(k)).collect())
    }

    fn set(&mut self, key: Lowercase, game: GameData) -> DaoResult<()> {
        let request = &self
            .client
            .put_item()
            .table_name(DYNAMO_TABLE_NAME)
            .item(DYNAMO_KEY_ATTRIBUTE, AttributeValue::S(key.value().to_string()))
            .item(DYNAMO_TTL_ATTRIBUTE, AttributeValue::N(DynamoDao::get_ttl().to_string()))
            .item(DYNAMO_GAME_ATTRIBUTE, AttributeValue::S(json!(game).to_string()));

        match self.runtime.block_on(request.clone().send()) {
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
