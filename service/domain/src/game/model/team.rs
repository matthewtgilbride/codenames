use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Display, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Team {
    Blue,
    Red,
}
