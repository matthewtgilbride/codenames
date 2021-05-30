use actix_web::{get, put, web, Responder, Scope};

use codenames_domain::game::model::{GuessRequest, PlayerRequest};

use crate::util::respond;
use crate::AppData;

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
    let player_request = PlayerRequest { player_name };
    respond(&data.service.clone().get(key, Some(player_request)))
}

#[put("/guess/{index}")]
async fn guess(
    path: web::Path<(String, String, usize)>,
    data: web::Data<AppData>,
) -> impl Responder {
    let (key, player_name, board_index) = path.clone();
    let guess_request = GuessRequest {
        player_name,
        board_index,
    };
    respond(&data.service.clone().guess(key, guess_request))
}

#[put("/leave")]
async fn leave_game(path: web::Path<(String, String)>, data: web::Data<AppData>) -> impl Responder {
    let (key, player_name) = path.clone();
    let player_request = PlayerRequest { player_name };
    respond(&data.service.clone().leave(key, player_request))
}
