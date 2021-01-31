use actix_web::{get, web, HttpResponse, Responder};

use crate::AppState;

#[get("/")]
pub async fn random_name(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(&data.service)
}
