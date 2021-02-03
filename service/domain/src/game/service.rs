use crate::dictionary::service::{Service as DictionaryService, WordGenerator};
use crate::game::board::service::{BoardGenerator, Service as BoardService};
use crate::game::dao::DAO;
use crate::game::model::{Game, Guess, GuessRequest, NewGameRequest, Player};
use crate::StdResult;

#[derive(Clone)]
pub struct Service {
    board_service: BoardService,
    dictionary_service: DictionaryService,
    dao: Box<dyn DAO>,
}

impl Service {
    pub fn new(
        word_generator: Box<dyn WordGenerator>,
        board_generator: Box<dyn BoardGenerator>,
        dao: Box<dyn DAO + Send + Sync>,
    ) -> StdResult<Service> {
        let dictionary_service = DictionaryService::new(word_generator)?;
        let board_service = BoardService::new(board_generator);
        Ok(Service {
            board_service,
            dictionary_service,
            dao,
        })
    }

    pub fn random_name(&self) -> StdResult<NewGameRequest> {
        let (first_name, last_name) = self.dictionary_service.new_word_pair()?;
        Ok(NewGameRequest {
            name: format!("{}-{}", first_name, last_name),
        })
    }

    pub fn new_game(&self, request: NewGameRequest) -> StdResult<Game> {
        let words = self.dictionary_service.new_word_set()?;
        let (board, first_team) = self.board_service.new_board(words)?;

        let game = Game::new(request.name, board, first_team);
        let _ = &self.clone().save(game.clone())?;

        Ok(game.clone())
    }

    pub fn join(&self, key: String, player: Player) -> StdResult<Game> {
        let game = &self.clone().get(key)?;
        let updated_game = game.clone().join(player)?;
        Ok(updated_game.clone())
    }

    pub fn leave(&self, key: String, player: Player) -> StdResult<Game> {
        let game = &self.clone().get(key)?;
        let updated_game = game.clone().leave(player.name.as_str());
        let _ = &self.clone().save(updated_game.clone())?;
        Ok(updated_game.clone())
    }

    pub fn guess(&self, key: String, guess: GuessRequest) -> StdResult<Game> {
        let game = &self.clone().get(key)?;
        game.players
            .iter()
            .cloned()
            .find(
                |Player {
                     name,
                     is_spy_master,
                     team,
                     ..
                 }| {
                    *name == guess.player_name && *is_spy_master == false && *team == game.turn
                },
            )
            .map_or_else(
                || Err("guess must be made by a valid player in the game (by name), on the team that matches the game's current turn, who is not a spy master".into()),
                |_| {
                    let updated_game = game.clone().guess(Guess {
                        board_index: guess.board_index,
                    })?;
                    let _ = &self.clone().save(updated_game.clone())?;
                    Ok(updated_game.clone())
                },
            )
    }

    pub fn undo_guess(&self, key: String) -> StdResult<Game> {
        let game = &self.clone().get(key)?;
        let updated_game = game.clone().undo_guess();
        let _ = &self.clone().save(updated_game.clone())?;
        Ok(updated_game.clone())
    }

    pub fn end_turn(&self, key: String) -> StdResult<Game> {
        let game = &self.clone().get(key)?;
        let updated_game = game.clone().end_turn();
        let _ = &self.clone().save(updated_game.clone())?;
        Ok(updated_game.clone())
    }

    pub fn get(&mut self, key: String) -> StdResult<Game> {
        self.dao.get(key.to_lowercase())
    }

    fn save(&mut self, game: Game) -> StdResult<()> {
        let key = game.name.to_lowercase();
        self.dao.set(key, game)
    }
}
