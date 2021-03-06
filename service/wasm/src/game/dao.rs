use codenames_domain::game::dao::{DaoError, DaoResult, DAO};
use codenames_domain::game::model::Game;

use wasmcloud_actor_keyvalue as kv;

#[derive(Clone)]
pub struct WasmKeyValueDao;

impl DAO for WasmKeyValueDao {
    fn get(&mut self, key: String) -> DaoResult<Game> {
        let result = kv::default()
            .get(key.clone())
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
    fn keys(&mut self) -> DaoResult<Vec<String>> {
        unimplemented!()
    }
    fn set(&mut self, _: String, _: Game) -> DaoResult<()> {
        unimplemented!()
    }
}
