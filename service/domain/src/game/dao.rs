use crate::game::model::Game;
use crate::StdResult;

pub trait DAO {
    fn get(&mut self, key: String) -> StdResult<Game>;
    fn set(&mut self, key: String, game: Game) -> StdResult<()>;
}
