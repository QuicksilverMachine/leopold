use actix_web::{Responder, HttpResponse, web};

use crate::api::schema;
use crate::commands;


pub async fn run_command(request: web::Json<schema::CommandRunRequest>) -> impl Responder {
    commands::run_command(&request.command, &request.args).await;
    HttpResponse::Ok().body("OK")
}
