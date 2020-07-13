use actix_web::{web, Responder};
use uuid::Uuid;

use super::schema;
use crate::tasks;

pub async fn run(request: web::Json<schema::TaskRunRequest>) -> impl Responder {
    let task_id = format!("{}:{}", &request.task_key, Uuid::new_v4().to_string());
    tasks::run(&request.app, &request.task_key, &task_id).await;
    web::Json(schema::TaskRunResponse {
        message: "OK".to_string(),
    })
}
