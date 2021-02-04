use actix_web::{get, post, put, web, HttpResponse, Responder};

use codenames_domain::game::model::{Game, GuessRequest, NewGameRequest, Player};
use codenames_domain::{ServiceError, ServiceResult};

use crate::AppData;

fn respond(result: &ServiceResult<Game>) -> impl Responder {
    match result {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(service_error) => match service_error {
            ServiceError::NotFound(msg) => HttpResponse::NotFound().body(msg),
            ServiceError::BadRequest(msg) => HttpResponse::BadRequest().body(msg),
            ServiceError::Unknown(msg) => HttpResponse::InternalServerError().body(msg),
        },
    }
}

#[post("/")]
pub async fn new_game(body: web::Json<NewGameRequest>, data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.new_game(body.into_inner()))
}

#[get("/{id}")]
pub async fn get_game(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.clone().get(path.clone()))
}

#[put("/{id}/join")]
pub async fn join_game(
    path: web::Path<String>,
    body: web::Json<Player>,
    data: web::Data<AppData>,
) -> impl Responder {
    respond(&data.service.clone().join(path.clone(), body.into_inner()))
}

#[put("/{id}/leave")]
pub async fn leave_game(
    path: web::Path<String>,
    body: web::Json<Player>,
    data: web::Data<AppData>,
) -> impl Responder {
    respond(&data.service.clone().leave(path.clone(), body.into_inner()))
}

#[put("/{id}/guess")]
pub async fn guess(
    path: web::Path<String>,
    body: web::Json<GuessRequest>,
    data: web::Data<AppData>,
) -> impl Responder {
    respond(&data.service.clone().guess(path.clone(), body.into_inner()))
}

#[put("/{id}/guess/undo")]
pub async fn undo_guess(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.clone().undo_guess(path.clone()))
}

#[put("/{id}/end-turn")]
pub async fn end_turn(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.clone().end_turn(path.clone()))
}
