use std::io::Result;
use actix_web::{App, HttpServer, web};

use crate::api;


pub async fn run() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/commands/run", web::post().to(api::commands::run_command))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
