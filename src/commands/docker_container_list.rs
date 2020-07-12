use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerList {
    description: Option<String>,
}

impl DockerContainerList {
    pub async fn run(&self) -> Result<(), CommandError> {
        let containers = docker::commands::container_list().await?;
        for container in containers {
            println!("\t{}", container.name)
        }
        Ok(())
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}
