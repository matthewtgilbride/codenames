use std::fmt;

use serde::__private::Formatter;

pub type StdResult<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

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
