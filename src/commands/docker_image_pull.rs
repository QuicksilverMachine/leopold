use serde::Deserialize;

use crate::errors::CommandError;
use crate::{docker, logger};

#[derive(Clone, Deserialize, Debug)]
pub struct DockerImagePull {
    description: Option<String>,
    name: String,
    version: String,
}

impl DockerImagePull {
    fn image(&self) -> String {
        format!("{}:{}", self.name, &self.version)
    }

    pub async fn run(&self, task_id: String) -> Result<(), CommandError> {
        docker::commands::image_pull(&self.image()).await?;
        logger::info_task(task_id.clone(), self.image());
        Ok(())
    }

    pub async fn revert(&self, task_id: String) -> Result<(), CommandError> {
        docker::commands::image_remove(&self.image(), true).await?;
        logger::info_task(task_id.clone(), self.image());
        Ok(())
    }
}
