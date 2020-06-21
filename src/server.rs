use std::io::Result;
use actix_web::{App, HttpServer, web};
use crate::api;


pub async fn run() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/docker/images", web::get().to(api::list_images))
            .route("/docker/images/pull", web::get().to(api::pull_image))
            .route("/docker/containers", web::get().to(api::list_containers))
            .route("/docker/version", web::get().to(api::version))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
