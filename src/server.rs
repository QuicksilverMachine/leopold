use actix_web::{web, App, HttpServer};
use std::io::Result;

use crate::api;

pub async fn run() -> Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/task/execute",
            web::post().to(api::tasks::handlers::execute),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
