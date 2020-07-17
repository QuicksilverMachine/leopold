use serde::Deserialize;

use crate::errors::CommandError;
use crate::{docker, logger};

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerCreate {
    description: Option<String>,
    image: String,
    name: String,
    mounts: Option<Vec<String>>,
    ports: Option<Vec<docker::models::DockerContainerPortBinding>>,
}

impl DockerContainerCreate {
    pub async fn run(&self, task_id: String) -> Result<(), CommandError> {
        docker::commands::container_create(
            &self.image,
            &self.name,
            self.mounts.as_ref().unwrap_or(&vec![]).clone(),
            self.ports.as_ref().unwrap_or(&vec![]).clone(),
        )
        .await?;
        logger::info_task(task_id.clone(), &self.name);
        Ok(())
    }

    pub async fn revert(&self, task_id: String) -> Result<(), CommandError> {
        docker::commands::container_remove(&self.name, true).await?;
        logger::info_task(task_id.clone(), &self.name);
        Ok(())
    }
}
