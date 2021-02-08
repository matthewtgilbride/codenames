use crate::game::board::model::BOARD_SIZE;
use crate::game::card::model::CardColor;
use crate::game::model::{Game, GameError, Guess, Team};

use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub team: Team,
    pub name: String,
    pub is_spy_master: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerCard {
    pub color: Option<CardColor>,
    pub word: String,
}

pub type PlayerBoard = [PlayerCard; BOARD_SIZE];

pub type PlayerGameResult = Result<PlayerGame, GameError>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerGame {
    player: Player,
    board: PlayerBoard,
}

impl PlayerGame {
    pub fn new(player_name: &String, game: &Game) -> PlayerGameResult {
        let player = game
            .players
            .iter()
            .find(|Player { name, .. }| *name == *player_name)
            .map_or_else(
                || Err(GameError::PlayerNotFound(player_name.clone())),
                |p| Ok(p.clone()),
            )?;

        let cards: Vec<PlayerCard> = game
            .board
            .iter()
            .enumerate()
            .map(|(index, card)| match player.is_spy_master {
                true => PlayerCard {
                    color: Some(card.color),
                    word: card.clone().word,
                },
                false => {
                    let maybe_card_color = game
                        .guesses
                        .iter()
                        .find(|Guess { board_index }| board_index == &index)
                        .map(|_| card.color);
                    PlayerCard {
                        color: maybe_card_color,
                        word: card.clone().word,
                    }
                }
            })
            .collect();

        let board: PlayerBoard = cards.try_into().unwrap();

        Ok(PlayerGame { player, board })
    }
}
