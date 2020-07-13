use serde::Deserialize;

use crate::errors::CommandError;
use crate::{docker, logger};

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerRemove {
    description: Option<String>,
    name: String,
    force: Option<bool>,
}

impl DockerContainerRemove {
    pub async fn run(&self, task_id: String) -> Result<(), CommandError> {
        docker::commands::container_remove(&self.name, self.force.unwrap_or(true)).await?;
        logger::task_info(
            task_id.clone(),
            format!("Container \"{}\" removed", &self.name),
        );
        Ok(())
    }

    pub async fn revert(&self, _: String) -> Result<(), CommandError> {
        Ok(())
    }
}
