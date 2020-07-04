use actix_web::{Responder, HttpResponse, web};

use serde_json::json;

use crate::tasks;
use super::schema;


pub async fn execute_task(request: web::Json<schema::TaskExecuteRequest>) -> impl Responder {
    match tasks::execute_task(&request.app, &request.task_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "OK"})),
        Err(error) => HttpResponse::Ok().json(json!({"message": error.message}))
    }
}
