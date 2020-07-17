use serde::Deserialize;

use crate::errors::CommandError;
use crate::{docker, logger};

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerList {
    description: Option<String>,
}

impl DockerContainerList {
    pub async fn run(&self, task_id: String) -> Result<(), CommandError> {
        let containers = docker::commands::container_list().await?;
        for container in containers {
            logger::info_task(
                task_id.clone(),
                container.name.strip_prefix('/').unwrap_or(&container.name),
            )
        }
        Ok(())
    }

    pub async fn revert(&self, _: String) -> Result<(), CommandError> {
        Ok(())
    }
}
