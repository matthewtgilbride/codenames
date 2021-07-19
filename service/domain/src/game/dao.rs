use dyn_clone::DynClone;

use crate::{game::model::GameData, DaoResult};

pub trait GameDao: DynClone + Send + Sync {
    fn get(&mut self, key: String) -> DaoResult<GameData>;
    fn keys(&mut self) -> DaoResult<Vec<String>>;
    fn set(&mut self, key: String, game: GameData) -> DaoResult<()>;
}

dyn_clone::clone_trait_object!(GameDao);
