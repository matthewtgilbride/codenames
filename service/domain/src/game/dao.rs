use dyn_clone::DynClone;

use crate::{game::model::GameData, DaoResult, Lowercase};

pub trait GameDao: DynClone + Send + Sync {
    fn get(&mut self, key: Lowercase) -> DaoResult<GameData>;
    fn keys(&mut self) -> DaoResult<Vec<Lowercase>>;
    fn set(&mut self, key: Lowercase, game: GameData) -> DaoResult<()>;
}

dyn_clone::clone_trait_object!(GameDao);
