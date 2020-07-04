use bollard::Docker;
use bollard::image::{ APIImages, CreateImageOptions, ListImagesOptions, RemoveImageOptions};
use bollard::container::{ListContainersOptions, APIContainers};
use chrono::{DateTime, Utc};
use futures::{StreamExt};
use serde::{Deserialize, Serialize};


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

#[derive(Debug)]
pub struct DockerError {
    pub message: String,
}

impl From<bollard::errors::Error> for DockerError {
    fn from(error: bollard::errors::Error) -> Self {
        DockerError{ message: error.to_string() }
    }
}

/// Generate a docker daemon connection
async fn docker_connection() -> Result<Docker, DockerError> {
    let docker = Docker::connect_with_local_defaults()?;
    match docker.version().await {
        Err(error) => Err(
            DockerError{ message: String::from(
                format!("Cannot connect to docker daemon: {:?}", error)) }
        ),
        _ => Ok(docker)
    }
}

/// Extract image name and version from api image
async fn image_name_and_version(tags: Vec<String>) -> Result<(String, String), DockerError>{
    match tags.first() {
        Some(first_tag) => {
            let items: Vec<String> = first_tag.split(":").map(|s| s.to_string()).collect();
            if items.len() == 2 {
                Ok((items[0].to_string(), items[1].to_string()))
            } else {
                Err(
                    DockerError{ message: String::from("Could not parse image name and version, invalid tag.") }
                )
            }
        },
        None => Err(
            DockerError{ message: String::from("Cannot parse image name and version, no tags found.") }
        ),
    }
}


/// Return docker api image converted to local image format
async fn api_to_image(api_image: APIImages) -> Result<Image, DockerError> {
    let tags = api_image.repo_tags.unwrap_or_default();
    let (name, version) = image_name_and_version(tags).await?;
    Ok(Image{
        name,
        tag: version,
        id: api_image.id,
        created: api_image.created,
        size: api_image.size,
    })
}

/// Return docker api inspect image converted to local image format
async fn inspect_to_image(api_image: bollard::image::Image) -> Result<Image, DockerError> {
    let tags = api_image.repo_tags;
    let (name, version) = image_name_and_version(tags).await?;
    Ok(Image{
        name,
        tag: version,
        id: api_image.id,
        created: api_image.created,
        size: api_image.size,
    })
}

/// Return docker api container converted to local container format
async fn api_to_container(api_container: APIContainers) -> Result<Container, DockerError> {
    Ok(Container {
        name: api_container.names[0].to_string(),
        created: api_container.created,
    })
}

/// Return list of downloaded images
pub async fn image_list() -> Result<Vec<Image>, DockerError> {
    let docker = docker_connection().await?;
    let images =  docker.list_images(None::<ListImagesOptions<String>>).await?;
    let mut image_list:Vec<Image> = Vec::new();
    for image in images {
        image_list.push(api_to_image(image).await?)
    }
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
    Ok(inspect_to_image(docker.inspect_image(image).await?).await?)
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
        container_list.push( api_to_container(container).await?);
    }
    Ok(container_list)
}

/// Return docker engine version
pub async fn version() -> Result<String, DockerError> {
    let docker = docker_connection().await?;
    let version = docker.version().await?.version;
    Ok(version)
}
