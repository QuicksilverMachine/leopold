use actix_web::{Responder, HttpResponse, web};

use crate::api::schema;
use crate::commands;


pub async fn execute_task(request: web::Json<schema::TaskExecuteRequest>) -> impl Responder {
    commands::execute_task(&request.app, &request.task_id).await;
    HttpResponse::Ok().body("OK")
}
