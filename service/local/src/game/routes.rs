use actix_web::{get, HttpResponse, post, put, Responder, web};

use codenames_domain::game::model::{Guess, NewGameRequest, Player};

use crate::AppData;

#[post("/")]
pub async fn new_game(body: web::Json<NewGameRequest>, data: web::Data<AppData>) -> impl Responder {
    let game_request = body.into_inner();
    let game = &data.service.new_game(game_request.clone()).unwrap();
    &data
        .service
        .clone()
        .save(game.clone().name.to_lowercase(), game.clone())
        .unwrap();
    HttpResponse::Ok().json(game)
}

#[get("/{id}")]
pub async fn get_game(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    let id = path.clone().to_lowercase();
    let game = &data.service.clone().get(id.clone()).unwrap();
    HttpResponse::Ok().json(game.clone())
}

#[put("/{id}/join")]
pub async fn join_game(
    path: web::Path<String>,
    body: web::Json<Player>,
    data: web::Data<AppData>,
) -> impl Responder {
    let game = &data.service.clone().get(path.clone()).unwrap();
    let updated_game = game.clone().join(body.into_inner()).unwrap();
    &data
        .service
        .clone()
        .save(path.clone(), updated_game)
        .unwrap();
    HttpResponse::Ok()
}

#[put("/{id}/leave")]
pub async fn leave_game(
    path: web::Path<String>,
    body: web::Json<Player>,
    data: web::Data<AppData>,
) -> impl Responder {
    let game = &data.service.clone().get(path.clone()).unwrap();
    let updated_game = game.clone().leave(body.into_inner().name.as_str());
    &data
        .service
        .clone()
        .save(path.clone(), updated_game)
        .unwrap();
    HttpResponse::Ok()
}

#[put("/{id}/guess")]
pub async fn guess(
    path: web::Path<String>,
    body: web::Json<Guess>,
    data: web::Data<AppData>,
) -> impl Responder {
    let game = &data.service.clone().get(path.clone()).unwrap();
    let updated_game = game.clone().guess(body.into_inner()).unwrap();
    &data
        .service
        .clone()
        .save(path.clone(), updated_game)
        .unwrap();
    HttpResponse::Ok()
}

#[put("/{id}/guess/undo")]
pub async fn undo_guess(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    let game = &data.service.clone().get(path.clone()).unwrap();
    let updated_game = game.clone().undo_guess();
    &data
        .service
        .clone()
        .save(path.clone(), updated_game)
        .unwrap();
    HttpResponse::Ok()
}

#[put("/{id}/end-turn")]
pub async fn end_turn(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    let game = &data.service.clone().get(path.clone()).unwrap();
    let updated_game = game.clone().end_turn();
    &data
        .service
        .clone()
        .save(path.clone(), updated_game)
        .unwrap();
    HttpResponse::Ok()
}
