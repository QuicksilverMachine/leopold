use std::io::Result;
use actix_web::{App, HttpServer, web};

use crate::api;


pub async fn run() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/task/execute", web::post().to(api::tasks::handlers::execute))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
