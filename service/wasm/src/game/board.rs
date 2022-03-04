use std::convert::TryInto;

use async_trait::async_trait;
use codenames_domain::{
    game::{
        board_service::BoardGenerator,
        model::{Board, Card, CardColor, CardState, Team},
    },
    ServiceError, ServiceResult,
};
use log::debug;
use wasmcloud_interface_numbergen::random_in_range;

#[derive(Clone)]
pub struct BoardGeneratorWasmCloud;

impl BoardGeneratorWasmCloud {
    async fn random_team(&self) -> ServiceResult<Team> {
        match random_in_range(0, 1)
            .await
            .map_err(|_| ServiceError::Unknown("could not get random number".into()))?
        {
            0 => Ok(Team::Blue),
            _ => Ok(Team::Red),
        }
    }
}

#[async_trait]
impl BoardGenerator for BoardGeneratorWasmCloud {
    async fn random_board(&self, words: [String; 25]) -> ServiceResult<(Board, Team)> {
        debug!("call: board.BoardGenerator.random_board");
        let first_team = self.random_team().await?;

        let mut indices: Vec<usize> = Vec::new();
        while indices.len() < 25 {
            let rand = random_in_range(0, 25)
                .await
                .map_err(|_| ServiceError::Unknown("could not get random number".into()))?
                as usize;

            debug!("selected index: {}", rand);

            if !indices.contains(&rand) {
                indices.push(rand)
            }
        }

        let mut initial_board: Vec<CardState> = words
            .iter()
            .map(|word| CardState {
                word: word.clone(),
                color: None,
            })
            .collect();

        indices
            .iter()
            .enumerate()
            .for_each(|(index, &random_index)| {
                let CardState { word, .. } = initial_board[random_index].clone();
                let color = match index {
                    0 => Some(CardColor::Death),
                    i if i < 8 => Some(CardColor::Neutral),
                    i if i < 16 => {
                        if first_team == Team::Blue {
                            Some(CardColor::Team(Team::Red))
                        } else {
                            Some(CardColor::Team(Team::Blue))
                        }
                    }
                    _ => Some(CardColor::Team(first_team)),
                };
                initial_board[random_index] = CardState { word, color }
            });

        let board: Vec<Card> = initial_board
            .iter()
            .map(|CardState { word, color }| Card {
                word: word.clone(),
                color: color.unwrap(),
            })
            .collect();

        Ok((board.try_into().unwrap(), first_team))
    }
}
