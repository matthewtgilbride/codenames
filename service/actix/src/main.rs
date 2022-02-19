extern crate env_logger;
#[macro_use]
extern crate serde_json;

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use codenames_domain::game::service::GameService;
use serde::{Deserialize, Serialize};

use crate::{
    dictionary::WordGeneratorRand,
    game::{board::BoardGeneratorRand, dao::DynamoDao, routes::routes as game_routes},
};

mod dictionary;
mod game;
mod util;

#[derive(Clone)]
pub struct AppData {
    service: GameService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let word_generator = Box::new(WordGeneratorRand);
    let board_generator = Box::new(BoardGeneratorRand);
    let dao = Box::new(DynamoDao::new().unwrap());

    let service = GameService::new(word_generator, board_generator, dao).unwrap();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let allowed_origins: Vec<String> = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "".into())
        .split(",")
        .map(|s| s.to_lowercase())
        .collect();

    HttpServer::new(move || {
        let mut cors = Cors::default().allow_any_header().allow_any_method();
        cors = allowed_origins
            .iter()
            .fold(cors, |c, o| c.allowed_origin(o));

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .data(
                AppData {
                    service: service.clone(),
                }
                .clone(),
            )
            .service(random_name)
            .service(game_routes("/game"))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

#[get("/")]
pub async fn random_name(data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(
        &data
            .service
            .random_name()
            .map(|g| GameNameBody::new(g))
            .unwrap(),
    )
}

#[derive(Serialize, Deserialize)]
struct GameNameBody {
    game_name: String,
}

impl GameNameBody {
    pub fn new(game_name: String) -> Self {
        Self { game_name }
    }
}

#[derive(Serialize, Deserialize)]
struct GameListBody {
    games: Vec<String>,
}

impl GameListBody {
    pub fn new(games: Vec<String>) -> Self {
        Self { games }
    }
}

#[derive(Serialize, Deserialize)]
struct ClueBody {
    word: String,
    amount: usize,
}
