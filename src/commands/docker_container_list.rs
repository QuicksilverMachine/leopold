use crate::docker;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DockerContainerList {
    description: String,
}

impl DockerContainerList {
    pub async fn execute(&self) {
        let containers = docker::container_list().await;
        for container in containers {
            println!("{}", container.name)
        }
    }
}
