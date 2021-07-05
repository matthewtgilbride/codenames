use crate::game::model::Team;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub team: Team,
    pub name: String,
    pub spymaster_secret: Option<String>,
}
