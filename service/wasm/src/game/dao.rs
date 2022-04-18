use async_trait::async_trait;
use codenames_domain::{
    game::{dao::GameDao, model::GameData},
    DaoError, DaoResult, Lowercase,
};
use kvdynamodb::{KeysRequest, KvDynamoDb, KvDynamoDbSender, SetRequest};
use wasmbus_rpc::{actor::prelude::WasmHost, common::Context};

pub struct WasmKeyValueDao {
    ctx: Box<Context>,
    kv: KvDynamoDbSender<WasmHost>,
}

impl Clone for WasmKeyValueDao {
    fn clone(&self) -> Self {
        return WasmKeyValueDao {
            ctx: self.ctx.clone(),
            kv: KvDynamoDbSender::new(),
        };
    }
}

impl WasmKeyValueDao {
    pub fn new(ctx: &Context) -> Self {
        Self {
            ctx: Box::new(ctx.clone()),
            kv: KvDynamoDbSender::new(),
        }
    }
}

#[async_trait]
impl GameDao for WasmKeyValueDao {
    async fn get(&mut self, key: Lowercase) -> DaoResult<GameData> {
        let result = self
            .kv
            .get(self.ctx.as_ref(), key.value())
            .await
            .map_err(|_| DaoError::Unknown("kv actor interface error".into()))?;
        match result.exists {
            true => serde_json::from_str(result.value.as_str()).map_err(|_| {
                DaoError::Unknown(format!(
                    "expected valid game json in db, got {}",
                    result.value
                ))
            }),
            false => DaoResult::Err(DaoError::NotFound(key.value().to_string())),
        }
    }
    async fn keys(&mut self) -> DaoResult<Vec<Lowercase>> {
        let keys_request = KeysRequest { cursor: None };

        let result = self
            .kv
            .keys(self.ctx.as_ref(), &keys_request)
            .await
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        Ok(result
            .keys
            .into_iter()
            .map(|s| Lowercase::new(s.as_str()))
            .collect())
    }
    async fn set(&mut self, key: Lowercase, game: GameData) -> DaoResult<()> {
        let set_request = SetRequest {
            key: key.value().to_string(),
            value: json!(game).to_string(),
            expires: 0,
        };
        self.kv
            .set(self.ctx.as_ref(), &set_request)
            .await
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        Ok(())
    }
}
