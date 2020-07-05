use actix_web::{web, Responder};

use super::schema;
use crate::tasks;

pub async fn run(request: web::Json<schema::TaskRunRequest>) -> impl Responder {
    tasks::run(&request.app, &request.task_id).await;
    web::Json(schema::TaskRunResponse {
        message: "OK".to_string(),
    })
}
