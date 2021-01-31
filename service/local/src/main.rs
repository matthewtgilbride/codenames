#[macro_use]
extern crate serde_json;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::dictionary::service::WordGeneratorRand;
use crate::game::board::service::BoardGeneratorRand;
use crate::game::dao::RedisDao;
use codenames_domain::game::service::Service;

mod dictionary;
mod game;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let word_generator = Box::new(WordGeneratorRand);
    let board_generator = Box::new(BoardGeneratorRand);
    let dao = Box::new(RedisDao::new().unwrap());

    let service = Service::new(word_generator, board_generator, dao).unwrap();

    HttpServer::new(move || App::new().data(service.clone()).service(random_name))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
pub async fn random_name(data: web::Data<Service>) -> impl Responder {
    HttpResponse::Ok().json(&data.random_name().unwrap())
}
