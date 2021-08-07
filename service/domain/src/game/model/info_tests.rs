use std::collections::HashMap;

use crate::{
    game::model::{GameError, GameInfo, Player, Team, Turn, TurnData},
    Lowercase,
};

const TEST_NAME: &str = "foo";

fn test_player(team: Team, is_spymaster: bool) -> Player {
    Player {
        name: TEST_NAME.to_string(),
        team,
        spymaster_secret: if is_spymaster {
            Some(TEST_NAME.to_string())
        } else {
            None
        },
    }
}

fn test_clue() -> (String, usize) {
    (TEST_NAME.to_string(), 1)
}

fn test_turn_data() -> TurnData {
    TurnData::new(test_player(Team::Blue, false), test_clue())
}

#[test]
fn new() {
    let info = GameInfo::new(TEST_NAME.to_string(), Team::Blue);
    assert_eq!(info.players, HashMap::new());
    assert_eq!(info.turns, vec![Turn::Pending(Team::Blue)]);
}

#[test]
fn start_turn_in_progress() {
    let info = GameInfo {
        name: TEST_NAME.to_string(),
        players: HashMap::new(),
        turns: vec![Turn::InProgress(test_turn_data())],
    };
    let result = info.start_turn(TEST_NAME.to_string(), test_clue());
    assert_eq!(result.err().unwrap(), GameError::TurnInProgress);
}

#[test]
fn start_turn_player_not_found() {
    let info = GameInfo::new(TEST_NAME.to_string(), Team::Blue);
    let result = info.start_turn(TEST_NAME.to_string(), test_clue());
    assert_eq!(
        result.err().unwrap(),
        GameError::PlayerNotFound(TEST_NAME.to_string())
    );
}

#[test]
fn start_turn_wrong_team() {
    let info = GameInfo {
        name: TEST_NAME.to_string(),
        turns: vec![Turn::Pending(Team::Blue)],
        players: [(Lowercase::new(TEST_NAME), test_player(Team::Red, true))]
            .iter()
            .cloned()
            .collect(),
    };
    let result = info.start_turn(TEST_NAME.to_string(), test_clue());
    assert_eq!(
        result.err().unwrap(),
        GameError::WrongTeam(TEST_NAME.to_string())
    );
}

#[test]
fn start_turn_not_a_spymaster() {
    let info = GameInfo {
        name: TEST_NAME.to_string(),
        turns: vec![Turn::Pending(Team::Blue)],
        players: [(Lowercase::new(TEST_NAME), test_player(Team::Blue, false))]
            .iter()
            .cloned()
            .collect(),
    };
    let result = info.start_turn(TEST_NAME.to_string(), test_clue());
    assert_eq!(
        result.err().unwrap(),
        GameError::NotASpymaster(TEST_NAME.to_string())
    );
}

#[test]
fn start_turn() {
    let player = test_player(Team::Blue, true);
    let info = GameInfo {
        name: TEST_NAME.to_string(),
        turns: vec![Turn::Pending(Team::Blue)],
        players: vec![(Lowercase::new(TEST_NAME), player.clone())]
            .into_iter()
            .collect(),
    };
    let result = info.clone().start_turn(TEST_NAME.to_string(), test_clue());
    assert_eq!(
        result.ok().unwrap(),
        GameInfo {
            turns: vec![Turn::InProgress(TurnData::new(player.clone(), test_clue()))],
            ..info.clone()
        }
    );
}

#[test]
fn end_pending_turn() {
    let info = GameInfo::new(TEST_NAME.to_string(), Team::Blue);
    assert_eq!(
        info.end_turn(),
        GameInfo {
            name: TEST_NAME.to_string(),
            players: HashMap::new(),
            turns: vec![Turn::Pending(Team::Red), Turn::Pending(Team::Blue)],
        }
    )
}

#[test]
fn end_in_progress_turn() {
    let info = GameInfo {
        name: TEST_NAME.to_string(),
        players: HashMap::new(),
        turns: vec![Turn::InProgress(test_turn_data())],
    };
    assert_eq!(
        info.end_turn(),
        GameInfo {
            name: TEST_NAME.to_string(),
            players: HashMap::new(),
            turns: vec![Turn::Pending(Team::Red), Turn::InProgress(test_turn_data())],
        }
    )
}

#[test]
fn end_second_turn() {
    let info = GameInfo {
        name: TEST_NAME.to_string(),
        players: HashMap::new(),
        turns: vec![Turn::Pending(Team::Red), Turn::InProgress(test_turn_data())],
    };
    assert_eq!(
        info.end_turn(),
        GameInfo {
            name: TEST_NAME.to_string(),
            players: HashMap::new(),
            turns: vec![
                Turn::Pending(Team::Blue),
                Turn::Pending(Team::Red),
                Turn::InProgress(test_turn_data())
            ],
        }
    )
}

#[test]
fn add_duplicate_player() {
    let info = GameInfo {
        name: TEST_NAME.to_string(),
        players: [(Lowercase::new(TEST_NAME), test_player(Team::Blue, false))]
            .iter()
            .cloned()
            .collect(),
        turns: vec![Turn::Pending(Team::Red)],
    };
    assert_eq!(
        info.add_player(test_player(Team::Red, true)).err().unwrap(),
        GameError::unique_player(TEST_NAME.to_string())
    )
}

#[test]
fn add_player() {
    let info = GameInfo {
        name: TEST_NAME.to_string(),
        players: [(Lowercase::new(TEST_NAME), test_player(Team::Blue, false))]
            .iter()
            .cloned()
            .collect(),
        turns: vec![Turn::Pending(Team::Red)],
    };
    let new_player = Player {
        team: Team::Blue,
        spymaster_secret: None,
        name: "bar".to_string(),
    };
    assert_eq!(
        info.clone().add_player(new_player.clone()).ok().unwrap(),
        GameInfo {
            players: [
                (Lowercase::new(TEST_NAME), test_player(Team::Blue, false)),
                (Lowercase::new("bar"), new_player)
            ]
            .iter()
            .cloned()
            .collect(),
            ..info.clone()
        }
    )
}

#[test]
pub fn remove_player_not_found() {
    assert_eq!(
        GameInfo::new(TEST_NAME.to_string(), Team::Blue)
            .remove_player(TEST_NAME)
            .err()
            .unwrap(),
        GameError::PlayerNotFound(TEST_NAME.to_string())
    )
}

#[test]
pub fn add_guess_pending_turn() {
    assert_eq!(
        GameInfo::new(TEST_NAME.to_string(), Team::Blue)
            .add_guess((TEST_NAME, 0))
            .err()
            .unwrap(),
        GameError::TurnPending
    )
}

#[test]
pub fn add_guess_player_not_found() {
    assert_eq!(
        GameInfo {
            name: TEST_NAME.to_string(),
            players: HashMap::new(),
            turns: vec![Turn::InProgress(test_turn_data())],
        }
        .add_guess((TEST_NAME, 0))
        .err()
        .unwrap(),
        GameError::PlayerNotFound(TEST_NAME.to_string())
    )
}

#[test]
pub fn add_guess_wrong_team() {
    assert_eq!(
        GameInfo {
            name: TEST_NAME.to_string(),
            players: [(Lowercase::new(TEST_NAME), test_player(Team::Red, false))]
                .iter()
                .cloned()
                .collect(),
            turns: vec![Turn::InProgress(test_turn_data())],
        }
        .add_guess((TEST_NAME, 0))
        .err()
        .unwrap(),
        GameError::WrongTeam(TEST_NAME.to_string())
    )
}

#[test]
pub fn add_guess_not_an_operative() {
    assert_eq!(
        GameInfo {
            name: TEST_NAME.to_string(),
            players: [(Lowercase::new(TEST_NAME), test_player(Team::Blue, true))]
                .iter()
                .cloned()
                .collect(),
            turns: vec![Turn::InProgress(test_turn_data())],
        }
        .add_guess((TEST_NAME, 0))
        .err()
        .unwrap(),
        GameError::NotAnOperative(TEST_NAME.to_string())
    )
}

#[test]
pub fn add_guess() {
    let info = GameInfo {
        name: TEST_NAME.to_string(),
        players: [(Lowercase::new(TEST_NAME), test_player(Team::Blue, false))]
            .iter()
            .cloned()
            .collect(),
        turns: vec![Turn::InProgress(test_turn_data())],
    };
    assert_eq!(
        info.clone().add_guess((TEST_NAME, 0)).ok().unwrap(),
        GameInfo {
            turns: vec![Turn::InProgress(TurnData {
                guesses: vec![(test_player(Team::Blue, false), 0)],
                ..test_turn_data()
            })],
            ..info
        }
    )
}
