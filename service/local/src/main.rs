extern crate env_logger;
#[macro_use]
extern crate serde_json;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use codenames_domain::game::service::Service;

use crate::dictionary::service::WordGeneratorRand;
use crate::game::board::service::BoardGeneratorRand;
use crate::game::dao::RedisDao;
use crate::game::routes::routes as game_routes;

mod dictionary;
mod game;
mod util;

#[derive(Clone)]
pub struct AppData {
    service: Service,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let word_generator = Box::new(WordGeneratorRand);
    let board_generator = Box::new(BoardGeneratorRand);
    let dao = Box::new(RedisDao::new().unwrap());

    let service = Service::new(word_generator, board_generator, dao).unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allowed_origin(std::env::var("APP_ORIGIN").unwrap().as_str());
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
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/")]
pub async fn random_name(data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(&data.service.random_name().unwrap())
}
