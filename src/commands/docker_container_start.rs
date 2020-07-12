use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerStart {
    description: Option<String>,
    name: String,
}

impl DockerContainerStart {
    pub async fn run(&self) -> Result<(), CommandError> {
        docker::commands::container_start(&self.name).await?;
        Ok(())
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}
