use codenames_domain::game::dao::GameDao;
use codenames_domain::game::model::Game;
use codenames_domain::{DaoError, DaoResult, Lowercase};
use async_trait::async_trait;
use wasmbus_rpc::actor::prelude::WasmHost;
use wasmbus_rpc::common::Context;

use wasmcloud_interface_keyvalue::{IncrementRequest, KeyValue, KeyValueSender, ListAddRequest, SetRequest};

const ALL_GAMES_KEY: &str = "GAME_KEYS";

#[derive(Clone)]
pub struct WasmKeyValueDao {
    ctx: Box<Context>,
    kv: KeyValueSender<WasmHost>
}

impl WasmKeyValueDao {
    pub fn new(ctx: &Context) -> Self {
        Self {
            ctx: Box::new(ctx.clone()),
            kv: KeyValueSender::new()
        }
    }
}

#[async_trait]
impl GameDao for WasmKeyValueDao {
    async fn get(&mut self, key: String) -> DaoResult<Game> {
        let result = self.kv
            .get(self.ctx.as_ref(), &key)
            .await
            .map_err(|_| DaoError::Unknown("kv actor interface error".into()))?;
        match result.exists {
            true => serde_json::from_str(result.value.as_str()).map_err(|_| {
                DaoError::Unknown(format!(
                    "expected valid game json in db, got {}",
                    result.value
                ))
            }),
            false => DaoResult::Err(DaoError::NotFound(key)),
        }
    }
    async fn keys(&mut self) -> DaoResult<Vec<String>> {
        let result = self.kv
            .list_range(self.ctx.as_ref(),ALL_GAMES_KEY.into(), 0, std::i32::MAX)
            .await
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        Ok(result.values)
    }
    async fn set(&mut self, key: String, game: Game) -> DaoResult<()> {
        let set_request = SetRequest {
            key,
            value: json!(game).to_string(),
            expires: 0
        };
        self.kv
            .set(self.ctx.as_ref(), &set_request)
            .await
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        let list_add_request = ListAddRequest {
            list_name: ALL_GAMES_KEY.into(),
            value: key.clone()
        };
        self.kv
            .list_add(self.ctx.as_ref(), &list_add_request)
            .await
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        Ok(())
    }
}
