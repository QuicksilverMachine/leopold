use bollard::Docker;
use bollard::image::{ APIImages, CreateImageOptions, ListImagesOptions, RemoveImageOptions};
use bollard::container::{ListContainersOptions, APIContainers};
use chrono::{DateTime, Utc};
use futures::{StreamExt};
use serde::{Deserialize, Serialize};

use crate::errors::DockerError;


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

/// Generate a docker daemon connection
async fn docker_connection() -> Result<Docker, DockerError> {
    let docker = Docker::connect_with_local_defaults()?;
    match docker.version().await {
        Err(error) => Err(
            DockerError{message: format!("Cannot connect to docker daemon: {:?}", error)}
        ),
        _ => Ok(docker)
    }
}

/// Convert APIImage to local image format
impl From<APIImages> for Image {
    fn from(api_image: APIImages) -> Self {
        let default_tag = "<none>:<none>".to_string();
        let default_tags = vec!["<none>:<none>".to_string()];
        let tags = api_image.repo_tags.unwrap_or(default_tags);
        let data = {
            tags
                .first()
                .unwrap_or(&default_tag)
                .split(":")
                .collect::<Vec<_>>()
        };
        Image{
            name: data[0].to_string(),
            tag: data[1].to_string(),
            id: api_image.id,
            created: api_image.created,
            size: api_image.size,
        }
    }
}

/// Convert bollard inspect image to local image format
impl From<bollard::image::Image> for Image {
    fn from(api_image: bollard::image::Image) -> Self {
        let default_tag = "<none>:<none>".to_string();
        let tags = api_image.repo_tags;
        let data = {
            tags
                .first()
                .unwrap_or(&default_tag)
                .split(":")
                .collect::<Vec<_>>()
        };
        Image{
            name: data[0].to_string(),
            tag: data[1].to_string(),
            id: api_image.id,
            created: api_image.created,
            size: api_image.size,
        }
    }
}

/// Convert APIContainers to local container format
impl From<APIContainers> for Container {
    fn from(api_container: APIContainers) -> Self {
        Container {
            name: api_container.names[0].to_string(),
            created: api_container.created,
        }
    }
}

/// Return list of downloaded images
pub async fn image_list() -> Result<Vec<Image>, DockerError> {
    let docker = docker_connection().await?;
    let images =  docker.list_images(None::<ListImagesOptions<String>>).await?;
    let mut image_list:Vec<Image> = Vec::new();
    for image in images {
        image_list.push(Image::from(image))
    }
    let image_list = image_list;
    Ok(image_list)
}

/// Pull an image from repository
pub async fn image_pull(image: &str) -> Result<Image, DockerError>{
    let docker = docker_connection().await?;

    // Pull image
    docker.create_image(
        Some(CreateImageOptions{from_image: image, ..Default::default()}),
        None,
    ).collect::<Vec<_>>().await;

    // Inspect image
    Ok(Image::from(docker.inspect_image(image).await?))
}

/// Remove an image from system
pub async fn image_remove(image: &str, force: bool) -> Result<(), DockerError>{
    let docker = docker_connection().await?;
    docker.remove_image(
        &image,
        Some(RemoveImageOptions { force, ..Default::default() }),
        None
    ).await?;
    Ok(())
}

/// Return list of created containers
pub async fn container_list() -> Result<Vec<Container>, DockerError> {
    let docker = docker_connection().await?;
    let containers = docker.list_containers(
        Some(ListContainersOptions::<String>{all: true, ..Default::default()})
    ).await?;
    let mut container_list:Vec<Container> = Vec::new();
    for container in containers {
        container_list.push(Container::from(container));
    }
    let container_list = container_list;
    Ok(container_list)
}

/// Return docker engine version
pub async fn version() -> Result<String, DockerError> {
    let docker = docker_connection().await?;
    let version = docker.version().await?.version;
    Ok(version)
}
