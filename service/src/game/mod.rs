mod api;
pub mod dao;
mod model;
pub mod routes;
pub mod service;

#[cfg(test)]
mod tests {
    use crate::game::api::{generate_board, generate_board_words};
    use crate::game::model::{Game, Player, Team};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::collections::HashSet;
    use std::iter;

    pub fn rand_dictionary(size: usize) -> HashSet<String> {
        iter::repeat(0)
            .take(size)
            .map(|_| {
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect()
            })
            .collect()
    }

    pub fn rand_game() -> Game {
        let (board, turn) =
            generate_board(generate_board_words(rand_dictionary(50)).unwrap()).unwrap();

        let players: Vec<Player> = vec![
            Player {
                team: Team::Blue,
                name: "foo".to_string(),
                is_spy_master: true,
            },
            Player {
                team: Team::Blue,
                name: "bar".to_string(),
                is_spy_master: false,
            },
            Player {
                team: Team::Red,
                name: "baz".to_string(),
                is_spy_master: true,
            },
            Player {
                team: Team::Red,
                name: "buzz".to_string(),
                is_spy_master: false,
            },
        ];

        players.iter().fold(
            Game::new("test".to_string(), board, turn).unwrap(),
            |game, p| game.join(p.clone()).unwrap(),
        )
    }
}
