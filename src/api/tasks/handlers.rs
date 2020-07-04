use actix_web::{Responder, HttpResponse, web};

use crate::tasks;
use super::schema;


pub async fn execute_task(request: web::Json<schema::TaskExecuteRequest>) -> impl Responder {
    tasks::execute_task(&request.app, &request.task_id).await;
    HttpResponse::Ok().body("OK")
}
