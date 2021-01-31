use crate::game::model::Game;
use crate::StdResult;
use dyn_clone::DynClone;

pub trait DAO: DynClone + Send + Sync {
    fn get(&mut self, key: String) -> StdResult<Game>;
    fn set(&mut self, key: String, game: Game) -> StdResult<()>;
}

dyn_clone::clone_trait_object!(DAO);
