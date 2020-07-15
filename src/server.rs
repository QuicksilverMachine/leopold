use std::io::Result;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use crate::{api, logger};

pub async fn run() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(&logger::server_log_format()))
            .route("/status", web::get().to(api::status::handlers::status))
            .route("/task/run", web::post().to(api::tasks::handlers::run))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
