#[macro_use]
extern crate enum_display_derive;

use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use serde::Serialize;

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

#[derive(Debug)]
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
