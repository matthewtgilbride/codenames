use actix_web::{get, post, put, web, Responder, Scope};

use codenames_domain::game::model::NewGameRequest;
use codenames_domain::game::model::Player;

use crate::game::player_routes::player_routes;
use crate::util::respond;
use crate::AppData;

pub fn routes(path: &str) -> Scope {
    web::scope(path)
        .service(find_games)
        .service(new_game)
        .service(game_routes())
}

#[get("")]
async fn find_games(data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.clone().find())
}

#[post("")]
async fn new_game(body: web::Json<NewGameRequest>, data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.new_game(body.into_inner()))
}

fn game_routes() -> Scope {
    web::scope("/{id}")
        .service(get_game)
        .service(join_game)
        .service(undo_guess)
        .service(end_turn)
        .service(player_routes("/{player}"))
}

#[get("")]
async fn get_game(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.clone().get(path.clone(), None))
}

#[put("/join")]
async fn join_game(
    path: web::Path<String>,
    body: web::Json<Player>,
    data: web::Data<AppData>,
) -> impl Responder {
    respond(&data.service.clone().join(path.clone(), body.into_inner()))
}

#[put("/guess/undo")]
async fn undo_guess(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.clone().undo_guess(path.clone()))
}

#[put("/end-turn")]
async fn end_turn(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.clone().end_turn(path.clone()))
}
