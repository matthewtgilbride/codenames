use actix_web::{get, put, web, Responder, Scope};

use crate::{util::respond, AppData};

pub fn player_routes(path: &str) -> Scope {
    web::scope(path)
        .service(get_player_game)
        .service(guess)
        .service(leave_game)
}

#[get("")]
async fn get_player_game(
    path: web::Path<(String, String)>,
    data: web::Data<AppData>,
) -> impl Responder {
    let (key, player_name) = path.clone();
    respond(&data.service.clone().get(key, Some(player_name)))
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
