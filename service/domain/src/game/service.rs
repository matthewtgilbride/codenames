use log::{info, warn};

use crate::dictionary::service::{Service as DictionaryService, WordGenerator};
use crate::game::board::service::{BoardGenerator, Service as BoardService};
use crate::game::dao::{DaoError, DAO};
use crate::game::model::{
    Game, GameList, GameVariant, GuessRequest, NewGameRequest, PlayerRequest,
};
use crate::game::model::{GameState, Player};
use crate::{ServiceError, ServiceResult, StdResult};

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

    pub fn random_name(&self) -> ServiceResult<NewGameRequest> {
        let (first_name, last_name) = self.dictionary_service.new_word_pair()?;
        Ok(NewGameRequest {
            game_name: format!("{}-{}", first_name, last_name),
        })
    }

    pub fn new_game(&self, request: NewGameRequest) -> ServiceResult<GameState> {
        let words = self.dictionary_service.new_word_set()?;
        let (board, first_team) = self.board_service.new_board(words)?;

        let game = Game::new(request.game_name, board, first_team);
        let _ = &self.clone().save(game.clone())?;

        Ok(game.clone().into())
    }

    pub fn join(&self, key: String, player: Player) -> ServiceResult<GameVariant> {
        let game = &self.clone()._get(key)?;
        let updated_game = game.clone().join(player.clone())?;
        let _ = &self.clone().save(updated_game.clone())?;
        Ok((player.clone(), updated_game).into())
    }

    pub fn leave(&self, key: String, req: PlayerRequest) -> ServiceResult<GameState> {
        let game = &self.clone()._get(key)?;
        let updated_game = game.clone().leave(req.player_name.as_str());
        let _ = &self.clone().save(updated_game.clone())?;
        Ok(updated_game.clone().into())
    }

    pub fn guess(&self, key: String, guess: GuessRequest) -> ServiceResult<GameState> {
        let game = &self.clone()._get(key)?;
        let updated_game = game.clone().guess(guess)?;
        let _ = &self.clone().save(updated_game.clone())?;
        Ok(updated_game.clone().into())
    }

    pub fn undo_guess(&self, key: String) -> ServiceResult<GameState> {
        let game = &self.clone()._get(key)?;
        let updated_game = game.clone().undo_guess();
        let _ = &self.clone().save(updated_game.clone())?;
        Ok(updated_game.clone().into())
    }

    pub fn end_turn(&self, key: String) -> ServiceResult<GameState> {
        let game = &self.clone()._get(key)?;
        let updated_game = game.clone().end_turn();
        let _ = &self.clone().save(updated_game.clone())?;
        Ok(updated_game.clone().into())
    }

    fn _get(&mut self, key: String) -> ServiceResult<Game> {
        self.dao.get(key.to_lowercase()).map_err(|e| {
            info!("{}", e);
            e.into()
        })
    }

    pub fn get(&mut self, key: String, req: Option<PlayerRequest>) -> ServiceResult<GameVariant> {
        let data = self._get(key)?;
        match req {
            None => Ok(GameVariant::State(data.into())),
            Some(PlayerRequest { player_name }) => {
                let player = data
                    .players
                    .get(player_name.as_str())
                    .ok_or(ServiceError::NotFound(format!("player: {}", player_name)))?;
                Ok((player.clone(), data).into())
            }
        }
    }

    pub fn find(&mut self) -> ServiceResult<GameList> {
        let games = self.dao.keys().map_err(|e| {
            warn!("{}", e);
            let de: DaoError = e.into();
            de
        })?;

        Ok(GameList { games })
    }

    fn save(&mut self, game: Game) -> ServiceResult<()> {
        let key = game.name.to_lowercase();
        self.dao.set(key, game).map_err(|e| {
            warn!("{}", e);
            e.into()
        })
    }
}
