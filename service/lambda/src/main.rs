use codenames_common::{
    dictionary::WordGeneratorRand,
    game::{board::BoardGeneratorRand, dao::DynamoDao, model::Player, service::GameService},
    ClueBody, GameListBody, GameNameBody, ServiceError,
};
use lambda_http::{
    http::{Method, StatusCode},
    run, service_fn, Body, Request, Response,
};
use percent_encoding::percent_decode_str;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    msg: String,
}

fn error_response(status: StatusCode, msg: String) -> Response<Body> {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&ErrorResponse { msg }).unwrap(),
        ))
        .unwrap()
}

fn json_response<T: Serialize>(value: &T) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(value).unwrap()))
        .unwrap()
}

fn service_error_response(err: &ServiceError) -> Response<Body> {
    match err {
        ServiceError::NotFound(msg) => error_response(StatusCode::NOT_FOUND, msg.clone()),
        ServiceError::BadRequest(msg) => error_response(StatusCode::BAD_REQUEST, msg.clone()),
        ServiceError::Unknown(msg) => {
            error_response(StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
        }
    }
}

fn respond<T: Serialize>(result: &Result<T, ServiceError>) -> Response<Body> {
    match result {
        Ok(value) => json_response(value),
        Err(err) => service_error_response(err),
    }
}

fn parse_body<T: serde::de::DeserializeOwned>(req: &Request) -> Result<T, Response<Body>> {
    serde_json::from_slice(req.body().as_ref())
        .map_err(|e| error_response(StatusCode::BAD_REQUEST, e.to_string()))
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .without_time()
        .init();

    let word_generator = Box::new(WordGeneratorRand);
    let board_generator = Box::new(BoardGeneratorRand);
    let dao = Box::new(DynamoDao::new().await.unwrap());
    let service = GameService::new(word_generator, board_generator, dao).unwrap();

    run(service_fn(|req: Request| {
        let service = service.clone();
        async move { Ok::<_, lambda_runtime::Error>(router(req, service).await) }
    }))
    .await
}

async fn router(req: Request, service: GameService) -> Response<Body> {
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    // Strip trailing slash for consistency
    let path = path.trim_end_matches('/');

    // Split path into segments and URL-decode them
    let segments: Vec<String> = path
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| percent_decode_str(s).decode_utf8_lossy().into_owned())
        .collect();
    let segments: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

    match (method, segments.as_slice()) {
        // GET / — random game name
        (Method::GET, []) => {
            let result = service.random_name().await.map(GameNameBody::new);
            respond(&result)
        }

        // GET /game — list all games
        (Method::GET, ["game"]) => {
            let result = service.clone().find().await.map(GameListBody::new);
            respond(&result)
        }

        // POST /game — create new game
        (Method::POST, ["game"]) => {
            let body: GameNameBody = match parse_body(&req) {
                Ok(b) => b,
                Err(r) => return r,
            };
            let result = service.new_game(body.game_name).await;
            respond(&result)
        }

        // GET /game/{id} — get game state
        (Method::GET, ["game", id]) => {
            let result = service.clone().get(id, &None, &None).await;
            respond(&result)
        }

        // PUT /game/{id}/join — join game
        (Method::PUT, ["game", id, "join"]) => {
            let player: Player = match parse_body(&req) {
                Ok(b) => b,
                Err(r) => return r,
            };
            let result = service.join(id.to_string(), player).await;
            respond(&result)
        }

        // PUT /game/{id}/end-turn — end turn
        (Method::PUT, ["game", id, "end-turn"]) => {
            let result = service.end_turn(id.to_string()).await;
            respond(&result)
        }

        // GET /game/{id}/{player} — get player game view
        (Method::GET, ["game", id, player_name]) => {
            let secret = extract_query_param(&req, "secret");
            let result = service
                .clone()
                .get(id, &Some(player_name.to_string()), &secret)
                .await;
            respond(&result)
        }

        // PUT /game/{id}/{player}/start-turn — spymaster starts turn
        (Method::PUT, ["game", id, player_name, "start-turn"]) => {
            let clue: ClueBody = match parse_body(&req) {
                Ok(b) => b,
                Err(r) => return r,
            };
            let result = service
                .start_turn(id.to_string(), player_name.to_string(), (clue.word, clue.amount))
                .await;
            respond(&result)
        }

        // PUT /game/{id}/{player}/guess/{index} — operative guesses
        (Method::PUT, ["game", id, player_name, "guess", index]) => {
            let board_index: usize = match index.parse() {
                Ok(i) => i,
                Err(_) => {
                    return error_response(StatusCode::BAD_REQUEST, "invalid index".into());
                }
            };
            let result = service
                .guess(id.to_string(), (player_name, board_index))
                .await;
            respond(&result)
        }

        // PUT /game/{id}/{player}/leave — leave game
        (Method::PUT, ["game", id, player_name, "leave"]) => {
            let result = service.leave(id.to_string(), player_name).await;
            respond(&result)
        }

        _ => error_response(StatusCode::NOT_FOUND, "not found".into()),
    }
}

fn extract_query_param(req: &Request, key: &str) -> Option<String> {
    req.uri().query().and_then(|q| {
        q.split('&')
            .filter_map(|pair| {
                let mut kv = pair.splitn(2, '=');
                let k = kv.next()?;
                let v = kv.next()?;
                if k == key { Some(v.to_string()) } else { None }
            })
            .next()
    })
}
