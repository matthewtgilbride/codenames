use redis::Connection;
use redis::{Commands, Value};

use codenames_domain::game::dao::{DaoError, DaoResult, DAO};
use codenames_domain::game::model::Game;
use codenames_domain::StdResult;

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
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let con = client.get_connection()?;
        Ok(RedisDao { con })
    }
}

impl DAO for RedisDao {
    fn get(&mut self, key: String) -> DaoResult<Game> {
        let value: Value = self
            .con
            .get(key.clone())
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
            Value::Nil => Err(DaoError::NotFound(key.clone())),
            _ => Err(DaoError::Unknown(
                "unexpected redis::Value type from get operation".to_string(),
            )),
        }?;

        serde_json::from_str(result.as_str()).map_err(|e| DaoError::Unknown(e.to_string()))
    }

    fn set(&mut self, key: String, game: Game) -> DaoResult<()> {
        self.con
            .set(key, json!(game).to_string())
            .map_err(|e| DaoError::Unknown(e.to_string()))
    }
}
