use dyn_clone::DynClone;

use crate::game::model::Game;
use crate::DaoResult;

pub trait GameDao: DynClone + Send + Sync {
    fn get(&mut self, key: String) -> DaoResult<Game>;
    fn keys(&mut self) -> DaoResult<Vec<String>>;
    fn set(&mut self, key: String, game: Game) -> DaoResult<()>;
}

dyn_clone::clone_trait_object!(GameDao);
