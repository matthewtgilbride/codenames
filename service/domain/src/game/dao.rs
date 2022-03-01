use async_trait::async_trait;
use dyn_clone::DynClone;

use crate::{game::model::GameData, DaoResult, Lowercase};

#[async_trait]
pub trait GameDao: DynClone + Send + Sync {
    async fn get(&mut self, key: Lowercase) -> DaoResult<GameData>;
    async fn keys(&mut self) -> DaoResult<Vec<Lowercase>>;
    async fn set(&mut self, key: Lowercase, game: GameData) -> DaoResult<()>;
}

dyn_clone::clone_trait_object!(GameDao);
