extern crate redis;

use self::redis::{Commands, Connection, RedisError};
use crate::game::model::Game;
use std::error::Error;

pub trait DAO {
    fn get(key: String) -> Result<Game, Box<dyn Error>>;
    fn set(key: String, game: Game) -> Result<(), Box<dyn Error>>;
}

pub struct RedisDao {}
impl RedisDao {
    pub fn get(key: String) -> Result<Game, Box<dyn Error>> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;
        let result: String = con.get(key)?;
        serde_json::from_str(result.as_str()).map_err(|e| e.into())
    }

    pub fn set(key: String, game: Game) -> Result<(), Box<dyn Error>> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;
        con.set(key, json!(game).to_string()).map_err(|e| e.into())
    }
}
