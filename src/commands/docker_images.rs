use crate::docker;


pub(crate) async fn list() {
    let images = docker::image_list().await;
    for image in images {
        println!("{}", image.name)
    }
}

pub(crate) async fn pull(name: &String, version: &String) {
    let image = format!("{}:{}", name, version);
    let _ = docker::image_pull(image).await;
    println!("Image pulled");
}
