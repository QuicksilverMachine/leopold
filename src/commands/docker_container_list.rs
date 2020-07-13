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
            logger::task_info(task_id.clone(), format!("{}", container.name))
        }
        Ok(())
    }

    pub async fn revert(&self, _: String) -> Result<(), CommandError> {
        Ok(())
    }
}
