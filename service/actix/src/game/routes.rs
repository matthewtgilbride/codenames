use actix_web::{get, post, put, web, Responder, Scope};
use codenames_domain::game::model::Player;

use crate::{
    game::player_routes::player_routes, util::respond, AppData, GameListBody, GameNameBody,
};

pub fn routes(path: &str) -> Scope {
    web::scope(path)
        .service(find_games)
        .service(new_game)
        .service(game_routes())
}

#[get("")]
async fn find_games(data: web::Data<AppData>) -> impl Responder {
    respond(
        &data
            .service
            .clone()
            .find()
            .map(|keys| GameListBody::new(keys)),
    )
}

#[post("")]
async fn new_game(body: web::Json<GameNameBody>, data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.new_game(body.into_inner().game_name))
}

fn game_routes() -> Scope {
    web::scope("/{id}")
        .service(get_game)
        .service(join_game)
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

#[put("/end-turn")]
async fn end_turn(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    respond(&data.service.clone().end_turn(path.clone()))
}
