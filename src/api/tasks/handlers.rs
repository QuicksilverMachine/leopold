use actix_web::{Responder, HttpResponse, web};

use serde_json::json;

use crate::tasks;
use super::schema;


pub async fn execute(request: web::Json<schema::TaskExecuteRequest>) -> impl Responder {
    tasks::execute(&request.app, &request.task_id).await;
    HttpResponse::Ok().json(json!({"message": "OK"}))
}
