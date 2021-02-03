use actix_web::{get, post, put, web, HttpResponse, Responder};

use codenames_domain::game::model::{GuessRequest, NewGameRequest, Player};

use crate::AppData;

#[post("/")]
pub async fn new_game(body: web::Json<NewGameRequest>, data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(&data.service.new_game(body.into_inner()).unwrap())
}

#[get("/{id}")]
pub async fn get_game(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(&data.service.clone().get(path.clone()).unwrap())
}

#[put("/{id}/join")]
pub async fn join_game(
    path: web::Path<String>,
    body: web::Json<Player>,
    data: web::Data<AppData>,
) -> impl Responder {
    HttpResponse::Ok().json(
        &data
            .service
            .clone()
            .join(path.clone(), body.into_inner())
            .unwrap(),
    )
}

#[put("/{id}/leave")]
pub async fn leave_game(
    path: web::Path<String>,
    body: web::Json<Player>,
    data: web::Data<AppData>,
) -> impl Responder {
    HttpResponse::Ok().json(
        &data
            .service
            .clone()
            .leave(path.clone(), body.into_inner())
            .unwrap(),
    )
}

#[put("/{id}/guess")]
pub async fn guess(
    path: web::Path<String>,
    body: web::Json<GuessRequest>,
    data: web::Data<AppData>,
) -> impl Responder {
    HttpResponse::Ok().json(
        &data
            .service
            .clone()
            .guess(path.clone(), body.into_inner())
            .unwrap(),
    )
}

#[put("/{id}/guess/undo")]
pub async fn undo_guess(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(&data.service.clone().undo_guess(path.clone()).unwrap())
}

#[put("/{id}/end-turn")]
pub async fn end_turn(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(&data.service.clone().end_turn(path.clone()).unwrap())
}
