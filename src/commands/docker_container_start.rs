use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerStart {
    description: Option<String>,
    name: String,
}

impl DockerContainerStart {
    pub async fn run(&self, _: String) -> Result<(), CommandError> {
        docker::commands::container_start(&self.name).await?;
        Ok(())
    }

    pub async fn revert(&self, _: String) -> Result<(), CommandError> {
        Ok(())
    }
}
