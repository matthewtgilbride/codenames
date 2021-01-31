use redis::Commands;
use redis::Connection;

use codenames_domain::game::dao::DAO;
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
    fn get(&mut self, key: String) -> StdResult<Game> {
        let result: String = self.con.get(key)?;
        serde_json::from_str(result.as_str()).map_err(|e| e.into())
    }

    fn set(&mut self, key: String, game: Game) -> StdResult<()> {
        self.con
            .set(key, json!(game).to_string())
            .map_err(|e| e.into())
    }
}
