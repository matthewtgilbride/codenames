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
        players: vec![(Lowercase::new(TEST_NAME), test_player(Team::Red, true))]
            .into_iter()
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
        players: vec![(Lowercase::new(TEST_NAME), test_player(Team::Blue, false))]
            .into_iter()
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
            turns: vec![Turn::InProgress(
                TurnData::new(player.clone(), test_clue(),)
            )],
            ..info.clone()
        }
    );
}
