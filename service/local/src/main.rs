#[macro_use]
extern crate serde_json;

use actix_web::{App, HttpServer};

use codenames_domain::game::service::Service;

use crate::actix_routes::random_name;
use crate::dictionary::service::WordGeneratorRand;
use crate::game::board::service::BoardGeneratorRand;
use crate::game::dao::RedisDao;

mod actix_routes;
mod dictionary;
mod game;

struct AppState {
    service: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let word_generator = Box::new(WordGeneratorRand);
    let board_generator = Box::new(BoardGeneratorRand);
    let dao = Box::new(RedisDao::new().unwrap());

    let _service = Service::new(word_generator, board_generator, dao).unwrap();

    HttpServer::new(|| {
        App::new()
            .data(AppState {
                service: String::from("foo"),
            })
            .service(random_name)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
