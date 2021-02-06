extern crate env_logger;
#[macro_use]
extern crate serde_json;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use codenames_domain::game::service::Service;

use crate::dictionary::service::WordGeneratorRand;
use crate::game::board::service::BoardGeneratorRand;
use crate::game::dao::RedisDao;
use crate::game::routes::{
    end_turn, find_games, get_game, guess, join_game, leave_game, new_game, undo_guess,
};

mod dictionary;
mod game;

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
        App::new()
            .wrap(Logger::default())
            .data(
                AppData {
                    service: service.clone(),
                }
                .clone(),
            )
            .service(random_name)
            .service(
                web::scope("/game")
                    .service(find_games)
                    .service(new_game)
                    .service(get_game)
                    .service(join_game)
                    .service(leave_game)
                    .service(guess)
                    .service(undo_guess)
                    .service(end_turn),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/")]
pub async fn random_name(data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(&data.service.random_name().unwrap())
}
