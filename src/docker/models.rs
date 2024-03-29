use bollard::models::{
    ContainerInspectResponse, ContainerSummary, ImageSummary, Mount, PortBinding,
};
use serde::{Deserialize, Serialize};

pub static DEFAULT_TIMEOUT: i64 = 10;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerPortBinding {
    pub host: String,
    pub internal_port: String,
    pub external_ports: Vec<String>,
}

pub struct APIDockerContainerPortBinding {
    pub internal_port: String,
    pub external_ports: Vec<PortBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainerMountBinding {
    pub target: String,
    pub source: String,
    pub read_only: bool,
}

/// Convert ImageSummary to local image format
impl From<ImageSummary> for Image {
    fn from(api_image: ImageSummary) -> Self {
        let default_tag = "<none>:<none>".to_string();
        let default_tags = vec!["<none>:<none>".to_string()];
        let tags = if api_image.repo_tags.is_empty() {
            default_tags
        } else {
            api_image.repo_tags
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
impl From<bollard::models::ImageInspect> for Image {
    fn from(api_image: bollard::models::ImageInspect) -> Self {
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
            id: api_image.id.unwrap(),
        }
    }
}

/// Convert ContainerSummary to local container format
impl From<ContainerSummary> for Container {
    fn from(api_container: ContainerSummary) -> Self {
        Container {
            name: api_container.names.unwrap_or_default()[0].to_string(),
        }
    }
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
            external_ports,
        }
    }
}

/// Convert ContainerSummary to local container format
impl From<ContainerInspectResponse> for Container {
    fn from(api_container: ContainerInspectResponse) -> Self {
        Container {
            name: api_container.name.unwrap_or_default(),
        }
    }
}

impl From<DockerContainerMountBinding> for Mount {
    fn from(mount_binding: DockerContainerMountBinding) -> Self {
        Mount {
            target: Some(mount_binding.target),
            source: Some(mount_binding.source),
            read_only: Some(mount_binding.read_only),
            // _type: Some(MountTypeEnum::BIND),
            ..Default::default()
        }
    }
}
