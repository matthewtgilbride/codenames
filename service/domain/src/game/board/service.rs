use dyn_clone::DynClone;

use crate::game::board::model::Board;
use crate::game::model::Team;
use crate::ServiceResult;

#[derive(Clone)]
pub struct Service {
    generator: Box<dyn BoardGenerator>,
}

impl Service {
    pub fn new(generator: Box<dyn BoardGenerator>) -> Service {
        Service { generator }
    }

    pub fn new_board(&self, words: [String; 25]) -> ServiceResult<(Board, Team)> {
        self.generator.random_board(words)
    }
}

pub trait BoardGenerator: DynClone + Send + Sync {
    fn random_board(&self, words: [String; 25]) -> ServiceResult<(Board, Team)>;
}

dyn_clone::clone_trait_object!(BoardGenerator);
