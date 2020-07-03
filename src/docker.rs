use bollard::Docker;
use bollard::image::{ListImagesOptions, CreateImageOptions, APIImages};
use bollard::container::{ListContainersOptions, APIContainers};
use serde::{Deserialize, Serialize};
use futures::{StreamExt};
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize)]
pub struct Image {
    pub name: String,
    pub tag: String,
    pub id: String,
    pub created: DateTime<Utc>,
    pub size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub created: DateTime<Utc>,
}

async fn api_to_image(api_image: APIImages) -> Image {
    let tags = api_image.repo_tags.unwrap();
    let full_name =  tags.first().unwrap().split(":").collect::<Vec<_>>();
    Image{
        name: full_name[0].to_string(),
        tag: full_name[1].to_string(),
        id: api_image.id,
        created: api_image.created,
        size: api_image.size,
    }
}

async fn inspect_to_image(api_image: bollard::image::Image) -> Image {
    let tags = api_image.repo_tags;
    let full_name =  tags.first().unwrap().split(":").collect::<Vec<_>>();
    Image{
        name: full_name[0].to_string(),
        tag: full_name[1].to_string(),
        id: api_image.id,
        created: api_image.created,
        size: api_image.size,
    }
}

async fn api_to_container(api_container: APIContainers) -> Container {
    Container {
        name: api_container.names[0].to_string(),
        created: api_container.created,
    }
}

pub async fn image_list() -> Vec<Image> {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let images =  docker.list_images(None::<ListImagesOptions<String>>).await.unwrap();
    let mut image_list:Vec<Image> = Vec::new();
    for image in images {
        image_list.push(api_to_image(image).await)
    }
    image_list
}


pub async fn image_pull(image: String) -> Image{
    let docker = Docker::connect_with_local_defaults().unwrap();

    // Pull image
    docker.create_image(
        Some(CreateImageOptions{from_image: image.clone(), ..Default::default()}),
        None,
    ).collect::<Vec<_>>().await;

    // Inspect image
    inspect_to_image(docker.inspect_image(image.clone().as_str()).await.unwrap()).await
}

pub async fn container_list () -> Vec<Container> {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let containers = docker.list_containers(
        Some(ListContainersOptions::<String>{all: true, ..Default::default()})
    ).await.unwrap();
    let mut container_list:Vec<Container> = Vec::new();
    for container in containers {
        container_list.push( api_to_container(container).await);
    }
    container_list
}

pub async fn version() -> String {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let version = docker.version().await.unwrap();
    version.version
}
