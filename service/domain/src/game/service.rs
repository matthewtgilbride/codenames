use log::{debug, info, warn};

use crate::{
    dictionary::{DictionaryService, WordGenerator},
    game::{
        board_service::{BoardGenerator, BoardService},
        dao::GameDao,
        model::{Game, GameData, GameState, Player},
    },
    DaoError, Lowercase, ServiceError, ServiceResult, StdResult,
};

#[derive(Clone)]
pub struct GameService {
    board_service: BoardService,
    dictionary_service: DictionaryService,
    dao: Box<dyn GameDao>,
}

impl GameService {
    pub fn new(
        word_generator: Box<dyn WordGenerator>,
        board_generator: Box<dyn BoardGenerator>,
        dao: Box<dyn GameDao + Send + Sync>,
    ) -> StdResult<GameService> {
        debug!("call: game.Service::new");
        let dictionary_service = DictionaryService::new(word_generator)?;
        let board_service = BoardService::new(board_generator);
        Ok(GameService {
            board_service,
            dictionary_service,
            dao,
        })
    }

    pub async fn random_name(&self) -> ServiceResult<String> {
        debug!("call: game.Service.random_name");
        let (first_name, last_name) = self.dictionary_service.new_word_pair().await?;
        Ok(format!("{}-{}", first_name, last_name))
    }

    pub async fn new_game(&self, game_name: String) -> ServiceResult<GameState> {
        let words = self.dictionary_service.new_word_set().await?;
        let (board, first_team) = self.board_service.new_board(words).await?;

        let game = GameData::new(game_name, board, first_team);
        let _ = &self.clone().save(game.clone()).await?;

        Ok(game.clone().into())
    }

    pub async fn join(&self, key: String, player: Player) -> ServiceResult<Game> {
        let game = &self.clone()._get(&key).await?;
        let updated_game = game.clone().join(player.clone())?;
        let _ = &self.clone().save(updated_game.clone()).await?;
        Ok((player.clone(), updated_game).into())
    }

    pub async fn leave(&self, key: String, player_name: &str) -> ServiceResult<GameState> {
        let game = &self.clone()._get(&key).await?;
        println!("got game {}", game.info.name());
        let updated_game = game.clone().leave(player_name)?;
        println!("left game {}", game.info.name());
        let _ = &self.clone().save(updated_game.clone()).await?;
        Ok(updated_game.clone().into())
    }

    pub async fn guess(&self, key: String, guess: (&str, usize)) -> ServiceResult<GameState> {
        let game = &self.clone()._get(&key).await?;
        let updated_game = game.clone().guess(guess)?;
        let _ = &self.clone().save(updated_game.clone()).await?;
        Ok(updated_game.clone().into())
    }

    pub async fn start_turn(
        &self,
        key: String,
        spymaster_name: String,
        clue: (String, usize),
    ) -> ServiceResult<GameData> {
        let game = &self.clone()._get(&key).await?;
        let updated_game = game.clone().start_turn(spymaster_name, clue)?;
        let _ = &self.clone().save(updated_game.clone()).await?;
        Ok(updated_game.clone())
    }

    pub async fn end_turn(&self, key: String) -> ServiceResult<GameState> {
        let game = &self.clone()._get(&key).await?;
        let updated_game = game.clone().end_turn();
        let _ = &self.clone().save(updated_game.clone()).await?;
        Ok(updated_game.clone().into())
    }

    async fn _get(&mut self, key: &str) -> ServiceResult<GameData> {
        self.dao.get(Lowercase::new(key)).await.map_err(|e| {
            info!("{}", e);
            e.into()
        })
    }

    pub async fn get(
        &mut self,
        key: &str,
        player_name: &Option<String>,
        spymaster_secret: &Option<String>,
    ) -> ServiceResult<Game> {
        let data = self._get(key).await?;
        match player_name {
            None => Ok(Game::State(data.into())),
            Some(player_name) => {
                let &player = data
                    .info
                    .players()
                    .iter()
                    .find(|&p| p.name.to_lowercase() == player_name.to_lowercase())
                    .ok_or(ServiceError::NotFound(format!("player: {}", player_name)))?;
                match (&player.spymaster_secret, spymaster_secret) {
                    (None, _) => Ok((player.clone(), data).into()),
                    (Some(player_secret), Some(provided_secret))
                        if player_secret == provided_secret =>
                    {
                        Ok((player.clone(), data).into())
                    }
                    _ => Ok(Game::State(data.into())),
                }
            }
        }
    }

    pub async fn find(&mut self) -> ServiceResult<Vec<String>> {
        let games = self
            .dao
            .keys()
            .await
            .map(|ls| ls.iter().map(|l| l.value().to_string()).collect())
            .map_err(|e| {
                warn!("{}", e);
                let de: DaoError = e.into();
                de
            })?;

        Ok(games)
    }

    async fn save(&mut self, game: GameData) -> ServiceResult<()> {
        let key = Lowercase::new(game.info.name());
        self.dao.set(key, game).await.map_err(|e| {
            warn!("{}", e);
            e.into()
        })
    }
}
