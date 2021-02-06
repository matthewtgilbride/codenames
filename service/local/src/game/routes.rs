use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use codenames_domain::game::model::{GuessRequest, NewGameRequest, Player};
use codenames_domain::{ServiceError, ServiceResult};

use crate::AppData;

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    msg: String,
}

impl ErrorResponse {
    fn new(msg: String) -> Self {
        ErrorResponse { msg }
    }
}

fn respond<T: Serialize>(result: &ServiceResult<T>) -> impl Responder {
    match result {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(service_error) => match service_error {
            ServiceError::NotFound(msg) => {
                HttpResponse::NotFound().json(ErrorResponse::new(msg.clone()))
            }
            ServiceError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse::new(msg.clone()))
            }
            ServiceError::Unknown(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse::new(msg.clone()))
            }
        },
    }
}

#[get("")]
pub async fn find_games(data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.clone().find())
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
