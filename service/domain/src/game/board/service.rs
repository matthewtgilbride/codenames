use crate::game::board::model::Board;
use crate::game::model::Team;
use crate::StdResult;

pub struct Service {
    generator: Box<dyn BoardGenerator + Send + Sync>,
}

impl Service {
    pub fn new(generator: Box<dyn BoardGenerator + Send + Sync>) -> Service {
        Service { generator }
    }

    pub fn new_board(&self, words: [String; 25]) -> StdResult<(Board, Team)> {
        self.generator.random_board(words)
    }
}

pub trait BoardGenerator {
    fn random_board(&self, words: [String; 25]) -> StdResult<(Board, Team)>;
}
