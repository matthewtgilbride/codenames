use actix_web::{get, put, web, Responder, Scope};
use serde::{Deserialize, Serialize};

use crate::{util::respond, AppData, ClueBody};

pub fn player_routes(path: &str) -> Scope {
    web::scope(path)
        .service(get_player_game)
        .service(start_turn)
        .service(guess)
        .service(leave_game)
}

#[derive(Serialize, Deserialize)]
struct SecretQuery {
    secret: Option<String>,
}

#[get("")]
async fn get_player_game(
    path: web::Path<(String, String)>,
    query: web::Query<SecretQuery>,
    data: web::Data<AppData>,
) -> impl Responder {
    let (key, player_name) = path.clone();
    respond(
        &data
            .service
            .clone()
            .get(&key, &Some(player_name), &query.secret),
    )
}

#[put("start-turn")]
async fn start_turn(
    path: web::Path<(String, String)>,
    body: web::Json<ClueBody>,
    data: web::Data<AppData>,
) -> impl Responder {
    let (key, player_name) = path.clone();
    let ClueBody { word, amount } = body.into_inner();
    respond(
        &data
            .service
            .clone()
            .start_turn(key, player_name, (word, amount)),
    )
}

#[put("/guess/{index}")]
async fn guess(
    path: web::Path<(String, String, usize)>,
    data: web::Data<AppData>,
) -> impl Responder {
    let (key, player_name, board_index) = path.clone();
    respond(
        &data
            .service
            .clone()
            .guess(key, (player_name.as_str(), board_index)),
    )
}

#[put("/leave")]
async fn leave_game(path: web::Path<(String, String)>, data: web::Data<AppData>) -> impl Responder {
    let (key, player_name) = path.clone();
    respond(&data.service.clone().leave(key, player_name.as_str()))
}
