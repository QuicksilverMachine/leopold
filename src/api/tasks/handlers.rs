use actix_web::{web, HttpResponse, Responder};

use serde_json::json;

use super::schema;
use crate::tasks;

pub async fn execute(request: web::Json<schema::TaskExecuteRequest>) -> impl Responder {
    tasks::execute(&request.app, &request.task_id).await;
    HttpResponse::Ok().json(json!({"message": "OK"}))
}
