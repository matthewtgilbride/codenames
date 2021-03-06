use wasmcloud_actor_keyvalue as kv;

use codenames_domain::game::dao::{DaoError, DaoResult, DAO};
use codenames_domain::game::model::Game;

const ALL_GAMES_KEY: &str = "GAME_KEYS";

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
        let result = kv::default()
            .range(ALL_GAMES_KEY.into(), 0, std::i32::MAX)
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        Ok(result.values)
    }
    fn set(&mut self, key: String, game: Game) -> DaoResult<()> {
        kv::default()
            .set(key.clone(), json!(game).to_string(), 0)
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        kv::default()
            .push(ALL_GAMES_KEY.into(), key.clone())
            .map_err(|e| DaoError::Unknown(e.to_string()))?;
        Ok(())
    }
}
