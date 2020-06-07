use bollard::Docker;
use bollard::image::{ListImagesOptions, CreateImageOptions, CreateImageResults};
use bollard::container::ListContainersOptions;
use tokio::runtime::Runtime;
use futures::{StreamExt};
use bollard::errors::Error;


pub fn image_list() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let images = Runtime::new().unwrap().block_on(
        docker.list_images(None::<ListImagesOptions<String>>)
    ).unwrap();
    for image in images {
        println!("-> {:?}", image);
    }
}

pub fn image_pull(image: &str) {
    let docker = Docker::connect_with_local_defaults().unwrap();

    // TODO: Remove if exists
    Runtime::new().unwrap().block_on(
        docker.create_image(
            Some(CreateImageOptions{ from_image: image, ..Default::default()}),
            None,
        ).collect::<Vec<_>>()
    );
    // TODO: Confirm successful
}

pub fn container_list () {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let containers = Runtime::new().unwrap().block_on(
        docker.list_containers(None::<ListContainersOptions<String>>)
    ).unwrap();
    for container in containers {
        println!("-> {:?}", container);
    }
}

pub fn version() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let version = Runtime::new().unwrap().block_on(
        docker.version()
    ).unwrap();
    println!("Docker version: {}", version.version);
}
