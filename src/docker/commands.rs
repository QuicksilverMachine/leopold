use std::collections::HashMap;

use bollard::container::{
    Config, CreateContainerOptions, InspectContainerOptions, ListContainersOptions,
    RemoveContainerOptions, RestartContainerOptions, StartContainerOptions, StopContainerOptions,
};
use bollard::image::{CreateImageOptions, ListImagesOptions, RemoveImageOptions};
use bollard::models::{HostConfig, Mount};
use bollard::Docker;
use futures::StreamExt;

use super::models::{
    APIDockerContainerPortBinding, Container, DockerContainerMountBinding,
    DockerContainerPortBinding, Image,
};
use crate::errors::DockerError;

/// Return docker engine version
pub async fn version() -> Result<String, DockerError> {
    let docker = docker_connection().await?;
    let version = docker.version().await?.version;
    Ok(version.unwrap())
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

/// Creates a new docker container
pub async fn container_create(
    image: &str,
    name: &str,
    mounts: Vec<DockerContainerMountBinding>,
    ports: Vec<DockerContainerPortBinding>,
) -> Result<Container, DockerError> {
    let docker = docker_connection().await?;

    let mut port_bindings = HashMap::new();
    for binding in ports {
        let binding = APIDockerContainerPortBinding::from(binding);
        port_bindings.insert(binding.internal_port, Some(binding.external_ports));
    }

    let mut mount_bindings = Vec::new();
    for binding in mounts {
        mount_bindings.push(Mount::from(binding));
    }

    let host_config = HostConfig {
        mounts: Some(mount_bindings),
        port_bindings: Some(port_bindings),
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

/// Starts docker container
pub async fn container_start(name: &str) -> Result<(), DockerError> {
    let docker = docker_connection().await?;
    docker
        .start_container(&name, None::<StartContainerOptions<String>>)
        .await?;
    Ok(())
}

/// Stops docker container
pub async fn container_stop(name: &str, timeout: i64) -> Result<(), DockerError> {
    let docker = docker_connection().await?;
    docker
        .stop_container(&name, Some(StopContainerOptions { t: timeout }))
        .await?;
    Ok(())
}

/// Restarts docker container
pub async fn container_restart(name: &str, timeout: i64) -> Result<(), DockerError> {
    let docker = docker_connection().await?;
    docker
        .restart_container(
            &name,
            Some(RestartContainerOptions {
                t: timeout as isize,
            }),
        )
        .await?;
    Ok(())
}
