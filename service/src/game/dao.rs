extern crate redis;

use crate::game::model::Game;
use crate::model::StandardResult;

use self::redis::{Commands, Connection};

pub trait DAO {
    fn get(&mut self, key: String) -> StandardResult<Game>;
    fn set(&mut self, key: String, game: Game) -> StandardResult<()>;
}

pub struct RedisDao {
    con: Connection,
}

impl RedisDao {
    pub fn new() -> StandardResult<RedisDao> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let con = client.get_connection()?;
        Ok(RedisDao { con })
    }
}

impl DAO for RedisDao {
    fn get(&mut self, key: String) -> StandardResult<Game> {
        let result: String = self.con.get(key)?;
        serde_json::from_str(result.as_str()).map_err(|e| e.into())
    }

    fn set(&mut self, key: String, game: Game) -> StandardResult<()> {
        self.con
            .set(key, json!(game).to_string())
            .map_err(|e| e.into())
    }
}
