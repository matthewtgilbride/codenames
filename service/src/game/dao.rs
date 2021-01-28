extern crate redis;

use self::redis::{Commands, Connection, RedisError};
use crate::game::model::Game;
use std::error::Error;

pub trait DAO {
    fn get(&mut self, key: String) -> Result<Game, Box<dyn Error>>;
    fn set(&mut self, key: String, game: Game) -> Result<(), Box<dyn Error>>;
}

pub struct RedisDao {
    con: Connection,
}

impl RedisDao {
    pub fn new() -> Result<RedisDao, Box<dyn Error>> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let con = client.get_connection()?;
        Ok(RedisDao { con })
    }
}

impl DAO for RedisDao {
    fn get(&mut self, key: String) -> Result<Game, Box<dyn Error>> {
        let result: String = self.con.get(key)?;
        serde_json::from_str(result.as_str()).map_err(|e| e.into())
    }

    fn set(&mut self, key: String, game: Game) -> Result<(), Box<dyn Error>> {
        self.con
            .set(key, json!(game).to_string())
            .map_err(|e| e.into())
    }
}
