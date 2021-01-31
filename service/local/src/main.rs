#[macro_use]
extern crate serde_json;

use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};

use crate::dictionary::service::WordGeneratorRand;
use crate::game::board::service::BoardGeneratorRand;
use crate::game::dao::RedisDao;
use codenames_domain::game::model::{NewGameRequest, Player};
use codenames_domain::game::service::Service;

mod dictionary;
mod game;

#[derive(Clone)]
pub struct AppData {
    service: Service,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let word_generator = Box::new(WordGeneratorRand);
    let board_generator = Box::new(BoardGeneratorRand);
    let dao = Box::new(RedisDao::new().unwrap());

    let service = Service::new(word_generator, board_generator, dao).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(
                AppData {
                    service: service.clone(),
                }
                .clone(),
            )
            .service(random_name)
            .service(new_game)
            .service(get_game)
            .service(join_game)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/")]
pub async fn random_name(data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(&data.service.random_name().unwrap())
}

#[post("/game")]
pub async fn new_game(req: web::Json<NewGameRequest>, data: web::Data<AppData>) -> impl Responder {
    let game_request = req.into_inner();
    let game = &data.service.new_game(game_request.clone()).unwrap();
    &data
        .service
        .clone()
        .save(game.clone().name.to_lowercase(), game.clone())
        .unwrap();
    HttpResponse::Ok().json(game)
}

#[get("/game/{id}")]
pub async fn get_game(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    let id = path.clone().to_lowercase();
    let game = &data.service.clone().get(id.clone()).unwrap();
    HttpResponse::Ok().json(game.clone())
}

#[put("/game/{id}/join")]
pub async fn join_game(
    path: web::Path<String>,
    player: web::Json<Player>,
    data: web::Data<AppData>,
) -> impl Responder {
    let game = &data.service.clone().get(path.clone()).unwrap();
    let updated_game = game.clone().join(player.into_inner()).unwrap();
    &data
        .service
        .clone()
        .save(path.clone(), updated_game)
        .unwrap();
    HttpResponse::Ok()
}
