use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerRemove {
    description: Option<String>,
    name: String,
    force: Option<bool>,
}

impl DockerContainerRemove {
    pub async fn run(&self) -> Result<(), CommandError> {
        docker::commands::container_remove(&self.name, self.force.unwrap_or(true)).await?;
        info!("Container \"{}\" removed.", &self.name);
        Ok(())
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}
