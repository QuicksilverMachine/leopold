use actix_web::{web, Responder};

use super::schema;

pub async fn status() -> impl Responder {
    web::Json(schema::StatusRequest {
        message: "OK".to_string(),
    })
}
