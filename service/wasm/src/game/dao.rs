use async_trait::async_trait;
use codenames_domain::{
    game::{dao::GameDao, model::GameData},
    DaoError, DaoResult, Lowercase,
};
use wasmbus_rpc::{actor::prelude::WasmHost, common::Context};
use wasmcloud_interface_keyvalue::{
    KeyValue, KeyValueSender, ListAddRequest, ListRangeRequest, SetRequest,
};

const ALL_GAMES_KEY: &str = "GAME_KEYS";

pub struct WasmKeyValueDao {
    ctx: Box<Context>,
    kv: KeyValueSender<WasmHost>,
}

impl Clone for WasmKeyValueDao {
    fn clone(&self) -> Self {
        return WasmKeyValueDao {
            ctx: self.ctx.clone(),
            kv: KeyValueSender::new(),
        };
    }
}

impl WasmKeyValueDao {
    pub fn new(ctx: &Context) -> Self {
        Self {
            ctx: Box::new(ctx.clone()),
            kv: KeyValueSender::new(),
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
        let list_range_request = ListRangeRequest {
            list_name: ALL_GAMES_KEY.into(),
            start: 0,
            stop: std::i32::MAX,
        };
        let result = self
            .kv
            .list_range(self.ctx.as_ref(), &list_range_request)
            .await
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        Ok(result
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
        let list_add_request = ListAddRequest {
            list_name: ALL_GAMES_KEY.into(),
            value: key.value().to_string(),
        };
        self.kv
            .list_add(self.ctx.as_ref(), &list_add_request)
            .await
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        Ok(())
    }
}
