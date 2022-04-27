#[macro_use]
extern crate serde_json;

use std::fmt::Debug;

use codenames_domain::{
    game::{model::Player, service::GameService},
    ClueBody, GameListBody, GameNameBody, ServiceError, ServiceResult, StdError,
};
use querystring::querify;
use serde_json::Value;
use urlencoding::decode;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::info;

use crate::{
    dictionary::WordGeneratorWasmCloud,
    game::{board::BoardGeneratorWasmCloud, dao::WasmKeyValueDao},
};

mod dictionary;
mod game;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct CodenamesActor {}

impl CodenamesActor {
    async fn do_handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let req_json = serde_json::to_string(req)
            .map_err(|_| RpcError::Ser(String::from("could not serialize request to log")))?;
        info!("*** Incoming Request ***");
        info!("{}", req_json);
        let word_generator = Box::new(WordGeneratorWasmCloud);
        let board_generator = Box::new(BoardGeneratorWasmCloud);
        let dao = Box::new(WasmKeyValueDao::new(ctx));
        let service =
            GameService::new(word_generator, board_generator, dao).map_err(|e| to_rpc_error(e))?;

        let &method = &req.method.as_str();
        let segments = path_segments(req);

        let routing_result: Result<Value, ServiceError> = do_routing(
            service,
            method,
            segments,
            req.body.as_slice(),
            &req.query_string,
        )
        .await;

        routing_result
            .map(|json| HttpResponse::json(json, 200))
            .unwrap_or_else(|se| {
                HttpResponse::json(
                    se.clone(),
                    match se {
                        ServiceError::BadRequest(_) => 400,
                        ServiceError::NotFound(_) => 404,
                        ServiceError::Unknown(_) => 500,
                    },
                )
            })
    }
}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for CodenamesActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        // let json: Value = Value::String(String::from("hello world"));
        // HttpResponse::json(json, 200)
        self.do_handle_request(ctx, req).await
    }
}

async fn do_routing(
    service: GameService,
    method: &str,
    segments: Vec<&str>,
    body_slice: &[u8],
    query_string: &str,
) -> Result<Value, ServiceError> {
    match (method, &segments[..]) {
        // get a random game key
        ("GET", []) => {
            debug_route("random game").await?;
            let json = json!(GameNameBody::new(service.random_name().await?));
            Ok(json!(json))
        }

        // create a game
        ("POST", ["game"]) => {
            debug_route("create game").await?;

            let body_str = std::str::from_utf8(body_slice)
                .map_err(|_| ServiceError::Unknown("error body utf8".to_string()))?;

            let body: GameNameBody =
                serde_json::from_str(body_str).map_err(|e| ServiceError::Unknown(e.to_string()))?;

            let game = service.new_game(body.game_name).await?;
            Ok(json!(game))
        }
        //
        // get a list of all games
        ("GET", ["game"]) => {
            debug_route("get all games").await?;
            let games = service.clone().find().await?;
            Ok(json!(GameListBody::new(games)))
        }
        //
        // // get an existing game
        ("GET", ["game", game_key]) => {
            debug_route("get game").await?;
            let game = service.clone().get(game_key, &None, &None).await?;
            Ok(json!(game))
        }
        //
        // join a game as a player
        ("PUT", ["game", game_key, "join"]) => {
            debug_route("join").await?;
            let player: Player = serde_json::from_str(
                std::str::from_utf8(body_slice)
                    .map_err(|e| ServiceError::Unknown(e.to_string()))?,
            )
            .map_err(|e| ServiceError::Unknown(e.to_string()))?;
            let updated_game = service.join(game_key.to_string(), player).await?;
            Ok(json!(updated_game))
        }
        //
        // end the current team's turn
        ("PUT", ["game", game_key, "end-turn"]) => {
            debug_route("end turn").await?;
            let updated_game = service.end_turn(game_key.to_string()).await?;
            Ok(json!(updated_game))
        }

        // get a player's view of the game
        ("GET", ["game", game_key, player_name_encoded]) => {
            debug_route("get player game").await?;
            let player_name =
                decode(player_name_encoded).map_err(|e| ServiceError::Unknown(e.to_string()))?;

            let spymaster_secret = &querify(query_string)
                .iter()
                .find(|(k, _)| k == &"secret")
                .map(|(_, v)| v.to_string());

            let game = service
                .clone()
                .get(game_key, &Some(player_name), spymaster_secret)
                .await?;

            Ok(json!(game))
        }

        // // guess a word
        ("PUT", ["game", game_key, player_name_encoded, "guess", index]) => {
            debug_route("guess").await?;
            let player_name =
                decode(player_name_encoded).map_err(|e| ServiceError::Unknown(e.to_string()))?;
            let board_index_result = index.parse::<usize>();
            match board_index_result {
                Ok(board_index) => {
                    let updated_game = service
                        .guess(game_key.to_string(), (player_name.as_str(), board_index))
                        .await?;
                    Ok(json!(updated_game))
                }
                Err(e) => Err(ServiceError::BadRequest(e.to_string())),
            }
        }

        // // leave a game
        ("PUT", ["game", game_key, player_name_encoded, "leave"]) => {
            debug_route("leave").await?;
            let player_name =
                decode(player_name_encoded).map_err(|e| ServiceError::Unknown(e.to_string()))?;
            let updated_game = service
                .clone()
                .leave(game_key.to_string(), player_name.as_str())
                .await?;
            Ok(json!(updated_game))
        }

        // start a turn
        ("PUT", ["game", game_key, player_name_encoded, "start-turn"]) => {
            debug_route("start turn").await?;
            let player_name =
                decode(player_name_encoded).map_err(|e| ServiceError::Unknown(e.to_string()))?;
            let ClueBody { word, amount } = serde_json::from_str(
                std::str::from_utf8(body_slice)
                    .map_err(|e| ServiceError::Unknown(e.to_string()))?,
            )
            .map_err(|e| ServiceError::Unknown(e.to_string()))?;
            let game = service
                .clone()
                .start_turn(game_key.to_string(), player_name, (word, amount))
                .await?;
            Ok(json!(game))
        }

        _ => Err(ServiceError::NotFound("unmatched route".to_string())),
    }
}

fn to_service_error(err: RpcError) -> ServiceError {
    ServiceError::Unknown(err.to_string())
}

fn to_rpc_error(err: StdError) -> RpcError {
    RpcError::Other(err.to_string())
}

pub fn path_segments(req: &HttpRequest) -> Vec<&str> {
    req.path
        .trim_end_matches('/')
        .split('/')
        .skip(1)
        .collect::<Vec<_>>()
}

async fn debug_route(msg: &str) -> ServiceResult<()> {
    log_stuff(format!("matched route: {}", msg)).await?;
    Ok(())
}

async fn log_stuff_inner(msg: String) -> RpcResult<()> {
    info!("{}", msg);
    Ok(())
}

async fn log_stuff(msg: String) -> ServiceResult<()> {
    log_stuff_inner(msg).await.map_err(to_service_error)
}
