use std::convert::TryInto;

use crate::game::model::{Card, CardColor, GameData, Player, Team, Turn, TurnData};

fn test_game() -> GameData {
    let cards: Vec<Card> = (0..25)
        .into_iter()
        .map(|i| {
            let color: CardColor = match i {
                blue if blue < 9 => CardColor::Team(Team::Blue),
                red if red < 17 => CardColor::Team(Team::Red),
                death if death < 18 => CardColor::Death,
                _ => CardColor::Neutral,
            };
            Card {
                color,
                word: i.to_string(),
            }
        })
        .collect();

    let game = GameData::new("test".to_string(), cards.try_into().unwrap(), Team::Blue);

    let players: Vec<Player> = vec![
        Player {
            team: Team::Blue,
            name: "foo".to_string(),
            spymaster_secret: Some("".into()),
        },
        Player {
            team: Team::Blue,
            name: "bar".to_string(),
            spymaster_secret: None,
        },
        Player {
            team: Team::Red,
            name: "baz".to_string(),
            spymaster_secret: Some("".into()),
        },
        Player {
            team: Team::Red,
            name: "buzz".to_string(),
            spymaster_secret: None,
        },
    ];

    players
        .iter()
        .fold(game, |game, p| game.join(p.clone()).unwrap())
}

#[test]
fn join() {
    let game = test_game();
    let game_clone = game.clone();
    let updated_game = game
        .join(Player {
            team: Team::Blue,
            name: "quz".to_string(),
            spymaster_secret: None,
        })
        .unwrap();

    assert_eq!(
        game_clone.info.players().len() + 1,
        updated_game.info.players().len()
    );

    let failed_update = updated_game.join(Player {
        team: Team::Red,
        name: "quz".to_string(),
        spymaster_secret: Some("".into()),
    });

    assert!(failed_update.is_err())
}

#[test]
fn leave() {
    let game = test_game();
    let game_clone = game.clone();
    let updated_game = game.leave("foo").unwrap();

    assert_eq!(
        game_clone.info.players().len() - 1,
        updated_game.info.players().len()
    )
}

#[test]
fn serialize() {
    let game = test_game();
    let j = serde_json::to_string_pretty(&game).unwrap();
    println!("{}", j);
    let game_started = game
        .start_turn("foo".to_string(), ("Foo".to_string(), 1))
        .unwrap();
    let j = serde_json::to_string_pretty(&game_started).unwrap();
    println!("{}", j);
}

#[test]
fn deserialize() {
    let game: GameData = serde_json::from_str(GAME_JSON).unwrap();
    assert_eq!(game.info.current_turn().clone(), Turn::Pending(Team::Blue));
    let game_started: GameData = serde_json::from_str(GAME_STARTED_JSON).unwrap();
    assert_eq!(
        game_started.info.current_turn().clone(),
        Turn::Started(TurnData::new(
            Player {
                name: "foo".to_string(),
                team: Team::Blue,
                spymaster_secret: Some("".to_string()),
            },
            ("Foo".to_string(), 1)
        ))
    );
}

#[test]
fn guess() {
    let game: GameData = test_game();
    let game_clone = game.clone();

    let player_name = game
        .info
        .players()
        .iter()
        .find(|&p| &p.team == game.info.current_turn().team() && p.spymaster_secret.is_none())
        .cloned()
        .unwrap()
        .clone()
        .name;

    let started_game = game
        .start_turn("foo".to_string(), ("bar".to_string(), 1))
        .unwrap();

    let updated_game = started_game.guess((player_name.as_str(), 0)).unwrap();

    assert_eq!(
        game_clone.info.guesses().len() + 1,
        updated_game.info.guesses().len()
    );

    let failed_update = updated_game.guess((player_name.as_str(), 0));

    assert!(failed_update.is_err())
}

const GAME_JSON: &str = r#"
{
  "name": "test",
  "players": {
    "baz": {
      "team": "Red",
      "name": "baz",
      "spymaster_secret": ""
    },
    "foo": {
      "team": "Blue",
      "name": "foo",
      "spymaster_secret": ""
    },
    "bar": {
      "team": "Blue",
      "name": "bar",
      "spymaster_secret": null
    },
    "buzz": {
      "team": "Red",
      "name": "buzz",
      "spymaster_secret": null
    }
  },
  "turns": [
    {
      "type": "Pending",
      "data": "Blue"
    }
  ],
  "board": [
    {
      "color": "Blue",
      "word": "0"
    },
    {
      "color": "Blue",
      "word": "1"
    },
    {
      "color": "Blue",
      "word": "2"
    },
    {
      "color": "Blue",
      "word": "3"
    },
    {
      "color": "Blue",
      "word": "4"
    },
    {
      "color": "Blue",
      "word": "5"
    },
    {
      "color": "Blue",
      "word": "6"
    },
    {
      "color": "Blue",
      "word": "7"
    },
    {
      "color": "Blue",
      "word": "8"
    },
    {
      "color": "Red",
      "word": "9"
    },
    {
      "color": "Red",
      "word": "10"
    },
    {
      "color": "Red",
      "word": "11"
    },
    {
      "color": "Red",
      "word": "12"
    },
    {
      "color": "Red",
      "word": "13"
    },
    {
      "color": "Red",
      "word": "14"
    },
    {
      "color": "Red",
      "word": "15"
    },
    {
      "color": "Red",
      "word": "16"
    },
    {
      "color": "Death",
      "word": "17"
    },
    {
      "color": "Neutral",
      "word": "18"
    },
    {
      "color": "Neutral",
      "word": "19"
    },
    {
      "color": "Neutral",
      "word": "20"
    },
    {
      "color": "Neutral",
      "word": "21"
    },
    {
      "color": "Neutral",
      "word": "22"
    },
    {
      "color": "Neutral",
      "word": "23"
    },
    {
      "color": "Neutral",
      "word": "24"
    }
  ]
}
"#;

const GAME_STARTED_JSON: &str = r#"
{
  "name": "test",
  "players": {
    "foo": {
      "team": "Blue",
      "name": "foo",
      "spymaster_secret": ""
    },
    "bar": {
      "team": "Blue",
      "name": "bar",
      "spymaster_secret": null
    },
    "baz": {
      "team": "Red",
      "name": "baz",
      "spymaster_secret": ""
    },
    "buzz": {
      "team": "Red",
      "name": "buzz",
      "spymaster_secret": null
    }
  },
  "turns": [
    {
      "type": "Started",
      "data": {
        "spymaster": {
          "team": "Blue",
          "name": "foo",
          "spymaster_secret": ""
        },
        "clue": [
          "Foo",
          1
        ],
        "guesses": []
      }
    }
  ],
  "board": [
    {
      "color": "Blue",
      "word": "0"
    },
    {
      "color": "Blue",
      "word": "1"
    },
    {
      "color": "Blue",
      "word": "2"
    },
    {
      "color": "Blue",
      "word": "3"
    },
    {
      "color": "Blue",
      "word": "4"
    },
    {
      "color": "Blue",
      "word": "5"
    },
    {
      "color": "Blue",
      "word": "6"
    },
    {
      "color": "Blue",
      "word": "7"
    },
    {
      "color": "Blue",
      "word": "8"
    },
    {
      "color": "Red",
      "word": "9"
    },
    {
      "color": "Red",
      "word": "10"
    },
    {
      "color": "Red",
      "word": "11"
    },
    {
      "color": "Red",
      "word": "12"
    },
    {
      "color": "Red",
      "word": "13"
    },
    {
      "color": "Red",
      "word": "14"
    },
    {
      "color": "Red",
      "word": "15"
    },
    {
      "color": "Red",
      "word": "16"
    },
    {
      "color": "Death",
      "word": "17"
    },
    {
      "color": "Neutral",
      "word": "18"
    },
    {
      "color": "Neutral",
      "word": "19"
    },
    {
      "color": "Neutral",
      "word": "20"
    },
    {
      "color": "Neutral",
      "word": "21"
    },
    {
      "color": "Neutral",
      "word": "22"
    },
    {
      "color": "Neutral",
      "word": "23"
    },
    {
      "color": "Neutral",
      "word": "24"
    }
  ]
}
"#;
