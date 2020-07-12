use std::io::Result;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use crate::api;

pub async fn run() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Starting Leopold server.");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/status", web::get().to(api::status::handlers::status))
            .route("/task/run", web::post().to(api::tasks::handlers::run))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
