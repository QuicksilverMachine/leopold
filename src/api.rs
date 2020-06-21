use actix_web::{Responder, HttpResponse};

use crate::containers;


pub async fn list_images() -> impl Responder {
    let images = containers::image_list().await;
    return HttpResponse::Ok().json(images);
}


pub async fn list_containers() -> impl Responder {
    let containers = containers::container_list().await;
    return HttpResponse::Ok().json(containers);
}


pub async fn pull_image() -> impl Responder {
    let name = "nginx:latest";
    let image = containers::image_pull(name).await;
    return HttpResponse::Ok().json(image);
}

pub async fn version() -> impl Responder {
    let version = containers::version().await;
    return HttpResponse::Ok().body(version);
}
