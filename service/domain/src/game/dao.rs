use std::error::Error;
use std::fmt;

use dyn_clone::DynClone;

use crate::game::model::Game;

#[derive(Debug)]
pub enum DaoError {
    NotFound(String),
    Unknown(String),
}

impl fmt::Display for DaoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match &self {
            DaoError::NotFound(msg) => format!("Not Found: {}", msg),
            DaoError::Unknown(msg) => format!("Unknown: {}", msg),
        };
        write!(f, "{}", format!("DAO Error: {}", msg))
    }
}

impl Error for DaoError {}

pub type DaoResult<T> = Result<T, DaoError>;

pub trait DAO: DynClone + Send + Sync {
    fn get(&mut self, key: String) -> DaoResult<Game>;
    fn keys(&mut self) -> DaoResult<Vec<String>>;
    fn set(&mut self, key: String, game: Game) -> DaoResult<()>;
}

dyn_clone::clone_trait_object!(DAO);
