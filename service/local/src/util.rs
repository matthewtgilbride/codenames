use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use codenames_domain::{ServiceError, ServiceResult};

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    msg: String,
}

impl ErrorResponse {
    fn new(msg: String) -> Self {
        ErrorResponse { msg }
    }
}

pub fn respond<T: Serialize>(result: &ServiceResult<T>) -> impl Responder {
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
