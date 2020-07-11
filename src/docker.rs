use bollard::container::{
    Config, CreateContainerOptions, InspectContainerOptions, ListContainersOptions,
    RemoveContainerOptions,
};
use bollard::image::{CreateImageOptions, ListImagesOptions, RemoveImageOptions};
use bollard::Docker;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::DockerError;
use bollard::models::{ContainerSummaryInner, HostConfig, ImageSummary, PortBinding};

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub name: String,
    pub tag: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Container {
    pub name: String,
}

/// Generate a docker daemon connection
async fn docker_connection() -> Result<Docker, DockerError> {
    let docker = Docker::connect_with_local_defaults()?;
    match docker.version().await {
        Err(error) => Err(DockerError {
            message: format!("Cannot connect to docker daemon: {:?}", error),
        }),
        _ => Ok(docker),
    }
}

/// Convert ImageSummary to local image format
impl From<ImageSummary> for Image {
    fn from(api_image: ImageSummary) -> Self {
        let default_tag = "<none>:<none>".to_string();
        let default_tags = vec!["<none>:<none>".to_string()];
        let tags = match api_image.repo_tags.is_empty() {
            true => default_tags.clone(),
            _ => api_image.repo_tags,
        };
        let data = {
            tags.first()
                .unwrap_or(&default_tag)
                .split(':')
                .collect::<Vec<_>>()
        };
        Image {
            name: data[0].to_string(),
            tag: data[1].to_string(),
            id: api_image.id,
        }
    }
}

/// Convert bollard inspect image to local image format
impl From<bollard::models::Image> for Image {
    fn from(api_image: bollard::models::Image) -> Self {
        let default_tag = "<none>:<none>".to_string();
        let default_tags = vec!["<none>:<none>".to_string()];
        let tags = api_image.repo_tags.unwrap_or(default_tags);
        let data = {
            tags.first()
                .unwrap_or(&default_tag)
                .split(':')
                .collect::<Vec<_>>()
        };
        Image {
            name: data[0].to_string(),
            tag: data[1].to_string(),
            id: api_image.id,
        }
    }
}

/// Convert ContainerSummaryInner to local container format
impl From<ContainerSummaryInner> for Container {
    fn from(api_container: ContainerSummaryInner) -> Self {
        Container {
            name: api_container.names.unwrap_or_default()[0].to_string(),
        }
    }
}

/// Convert ContainerSummaryInner to local container format
impl From<bollard::models::ContainerInspectResponse> for Container {
    fn from(api_container: bollard::models::ContainerInspectResponse) -> Self {
        Container {
            name: api_container.name.unwrap_or_default(),
        }
    }
}

/// Return list of downloaded images
pub async fn image_list() -> Result<Vec<Image>, DockerError> {
    let docker = docker_connection().await?;
    let images = docker
        .list_images(None::<ListImagesOptions<String>>)
        .await?;
    let mut image_list: Vec<Image> = Vec::new();
    for image in images {
        image_list.push(Image::from(image))
    }
    let image_list = image_list;
    Ok(image_list)
}

/// Pull an image from repository
pub async fn image_pull(image: &str) -> Result<Image, DockerError> {
    let docker = docker_connection().await?;

    // Pull image
    docker
        .create_image(
            Some(CreateImageOptions {
                from_image: image,
                ..Default::default()
            }),
            None,
            None,
        )
        .collect::<Vec<_>>()
        .await;

    // Inspect image
    Ok(Image::from(docker.inspect_image(image).await?))
}

/// Remove an image from system
pub async fn image_remove(image: &str, force: bool) -> Result<(), DockerError> {
    let docker = docker_connection().await?;
    docker
        .remove_image(
            &image,
            Some(RemoveImageOptions {
                force,
                ..Default::default()
            }),
            None,
        )
        .await?;
    Ok(())
}

/// Return list of created containers
pub async fn container_list() -> Result<Vec<Container>, DockerError> {
    let docker = docker_connection().await?;
    let containers = docker
        .list_containers(Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await?;
    let mut container_list: Vec<Container> = Vec::new();
    for container in containers {
        container_list.push(Container::from(container));
    }
    Ok(container_list)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerPortBinding {
    host: String,
    internal_port: String,
    external_ports: Vec<String>,
}

struct APIDockerContainerPortBinding {
    internal_port: String,
    external_ports: Vec<PortBinding>,
}

impl From<DockerContainerPortBinding> for APIDockerContainerPortBinding {
    fn from(port_bindings: DockerContainerPortBinding) -> Self {
        let mut external_ports = Vec::new();
        for port in port_bindings.external_ports {
            external_ports.push(PortBinding {
                host_ip: Some(port_bindings.host.to_string()),
                host_port: Some(port),
            });
        }

        APIDockerContainerPortBinding {
            internal_port: port_bindings.internal_port,
            external_ports: external_ports.clone(),
        }
    }
}

/// Creates a new docker container
pub async fn container_create(
    image: &str,
    name: &str,
    _mounts: Vec<String>,
    ports: Vec<DockerContainerPortBinding>,
) -> Result<Container, DockerError> {
    let docker = docker_connection().await?;

    let mut port_bindings = HashMap::new();
    for binding in ports {
        let binding = APIDockerContainerPortBinding::from(binding);
        port_bindings.insert(binding.internal_port, Some(binding.external_ports));
    }

    let host_config = HostConfig {
        port_bindings: Some(port_bindings.clone()),
        network_mode: Some("bridge".to_string()),
        ..Default::default()
    };

    let create_response = docker
        .create_container(
            Some(CreateContainerOptions { name }),
            Config {
                image: Some(image.to_string()),
                host_config: Some(host_config),
                ..Default::default()
            },
        )
        .await?;

    let inspect_data = Container::from(
        docker
            .inspect_container(&create_response.id, None::<InspectContainerOptions>)
            .await?,
    );
    Ok(inspect_data)
}

/// Removes a docker container
pub async fn container_remove(name: &str, force: bool) -> Result<(), DockerError> {
    let docker = docker_connection().await?;

    docker
        .remove_container(
            name,
            Some(RemoveContainerOptions {
                force,
                ..Default::default()
            }),
        )
        .await?;

    Ok(())
}

/// Return docker engine version
pub async fn version() -> Result<String, DockerError> {
    let docker = docker_connection().await?;
    let version = docker.version().await?.version;
    Ok(version)
}
