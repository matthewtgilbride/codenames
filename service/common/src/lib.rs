#[macro_use]
extern crate enum_display_derive;
extern crate serde_json;

use std::{error::Error, fmt, fmt::Formatter};

use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

pub mod dictionary;
pub mod game;

pub type StdError = Box<dyn std::error::Error + Sync + Send>;
pub type StdResult<T> = std::result::Result<T, StdError>;

#[derive(Debug, Serialize, Clone)]
pub enum ServiceError {
    BadRequest(String),
    NotFound(String),
    Unknown(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match &self {
            ServiceError::BadRequest(msg) => format!("Bad Request: {}", msg),
            ServiceError::NotFound(msg) => format!("Not Found: {}", msg),
            ServiceError::Unknown(msg) => format!("Unknown: {}", msg),
        };
        write!(f, "{}", format!("Service Error: {}", msg))
    }
}

impl From<DaoError> for ServiceError {
    fn from(dao_error: DaoError) -> Self {
        match dao_error {
            DaoError::NotFound(msg) => ServiceError::NotFound(msg),
            DaoError::Unknown(msg) => ServiceError::Unknown(msg),
        }
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(serde_err: serde_json::Error) -> Self {
        ServiceError::BadRequest(serde_err.to_string())
    }
}

impl From<&str> for ServiceError {
    fn from(unknown: &str) -> Self {
        Self::Unknown(unknown.to_string())
    }
}

impl Error for ServiceError {}

pub type ServiceResult<T> = Result<T, ServiceError>;

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

#[derive(Debug, Eq, PartialEq)]
pub struct UniqueError {
    entity_name: String,
    field_name: String,
    value: String,
}

impl UniqueError {
    pub fn new(entity_name: String, field_name: String, value: String) -> UniqueError {
        UniqueError {
            entity_name,
            field_name,
            value,
        }
    }
}

impl fmt::Display for UniqueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!(
            "{} values must be unique within a single {}, and the provided value already exists: {}",
            self.field_name, self.entity_name, self.value
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lowercase {
    value: String,
}

impl Lowercase {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_lowercase(),
        }
    }
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

impl Serialize for Lowercase {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value())
    }
}

impl<'de> Deserialize<'de> for Lowercase {
    fn deserialize<D>(deserializer: D) -> Result<Lowercase, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(LowercaseVisitor)
    }
}

struct LowercaseVisitor;

impl<'de> Visitor<'de> for LowercaseVisitor {
    type Value = Lowercase;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string which will be transformed to lowercase")
    }

    fn visit_str<E>(self, value: &str) -> Result<Lowercase, E>
    where
        E: de::Error,
    {
        Ok(Lowercase::new(value).clone())
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameNameBody {
    pub game_name: String,
}

impl GameNameBody {
    pub fn new(game_name: String) -> Self {
        Self { game_name }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameListBody {
    pub games: Vec<String>,
}

impl GameListBody {
    pub fn new(games: Vec<String>) -> Self {
        Self { games }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClueBody {
    pub word: String,
    pub amount: usize,
}
