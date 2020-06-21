use actix_web::{Responder, HttpResponse};
use serde_json::json;

use crate::containers;


pub async fn list_images() -> impl Responder {
    return HttpResponse::Ok().json(
        containers::image_list().await
    );
}

pub async fn list_containers() -> impl Responder {
    return HttpResponse::Ok().json(
        containers::container_list().await
    );
}

pub async fn pull_image() -> impl Responder {
    let name = "nginx:latest";
    return HttpResponse::Ok().json(
        containers::image_pull(name).await
    );
}

pub async fn version() -> impl Responder {
    return HttpResponse::Ok().json(
        json!({
            "version": containers::version().await
        })
    );
}
