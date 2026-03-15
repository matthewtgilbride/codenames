use async_trait::async_trait;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::{Duration, Utc};
use codenames_domain::{
    game::{dao::GameDao, model::GameData},
    DaoError,
    DaoError::NotFound,
    DaoResult, Lowercase, StdResult,
};

const DYNAMO_TABLE_NAME: &str = "codenames";
const DYNAMO_KEY_ATTRIBUTE: &str = "key";
const DYNAMO_TTL_ATTRIBUTE: &str = "ttl";
const DYNAMO_GAME_ATTRIBUTE: &str = "game";

#[derive(Clone)]
pub struct DynamoDao {
    client: Client,
}

impl DynamoDao {
    pub async fn new() -> StdResult<DynamoDao> {
        let shared_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = Client::new(&shared_config);
        Ok(DynamoDao { client })
    }

    fn get_ttl() -> i64 {
        Utc::now().timestamp() + Duration::days(1).num_seconds()
    }
}

#[async_trait]
impl GameDao for DynamoDao {
    async fn get(&mut self, key: Lowercase) -> DaoResult<GameData> {
        let result = self
            .client
            .get_item()
            .table_name(DYNAMO_TABLE_NAME)
            .key(
                DYNAMO_KEY_ATTRIBUTE,
                AttributeValue::S(key.value().to_string()),
            )
            .send()
            .await
            .map_err(|e| DaoError::Unknown(e.to_string()))?;

        let item = result.item.ok_or(NotFound(key.value().to_string()))?;
        let attribute = item.get(DYNAMO_GAME_ATTRIBUTE).ok_or(DaoError::Unknown(
            "could not find game attribute on dynamo result".into(),
        ))?;
        let game_string = attribute
            .as_s()
            .map_err(|_| DaoError::Unknown("could not get game as string".into()))?;

        serde_json::from_str(game_string).map_err(|e| DaoError::Unknown(e.to_string()))
    }

    async fn keys(&mut self) -> DaoResult<Vec<Lowercase>> {
        let result = self
            .client
            .scan()
            .table_name(DYNAMO_TABLE_NAME)
            .projection_expression("#k")
            .expression_attribute_names("#k", DYNAMO_KEY_ATTRIBUTE)
            .send()
            .await
            .map_err(|e| DaoError::Unknown(e.to_string()))?;

        let keys: Vec<String> = result
            .items()
            .iter()
            .map(|i| {
                i.get(DYNAMO_KEY_ATTRIBUTE)
                    .expect(format!("No {} field in response", DYNAMO_KEY_ATTRIBUTE).as_str())
            })
            .map(|a| {
                a.as_s()
                    .expect(format!("{} field was not a string", DYNAMO_KEY_ATTRIBUTE).as_str())
            })
            .cloned()
            .collect();

        Ok(keys.iter().map(|k| Lowercase::new(k)).collect())
    }

    async fn set(&mut self, key: Lowercase, game: GameData) -> DaoResult<()> {
        self.client
            .put_item()
            .table_name(DYNAMO_TABLE_NAME)
            .item(
                DYNAMO_KEY_ATTRIBUTE,
                AttributeValue::S(key.value().to_string()),
            )
            .item(
                DYNAMO_TTL_ATTRIBUTE,
                AttributeValue::N(DynamoDao::get_ttl().to_string()),
            )
            .item(
                DYNAMO_GAME_ATTRIBUTE,
                AttributeValue::S(serde_json::to_string(&game).map_err(|e| DaoError::Unknown(e.to_string()))?),
            )
            .send()
            .await
            .map(|_| ())
            .map_err(|e| DaoError::Unknown(e.to_string()))
    }
}
