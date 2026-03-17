use crate::game::model::{Card, CardState};

pub const BOARD_SIZE: usize = 25;
pub type Board = [Card; BOARD_SIZE];
pub type BoardState = [CardState; BOARD_SIZE];
