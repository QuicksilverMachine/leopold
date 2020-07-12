use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerCreate {
    description: Option<String>,
    image: String,
    name: String,
    mounts: Option<Vec<String>>,
    ports: Option<Vec<docker::models::DockerContainerPortBinding>>,
}

impl DockerContainerCreate {
    pub async fn run(&self) -> Result<(), CommandError> {
        docker::commands::container_create(
            &self.image,
            &self.name,
            self.mounts.as_ref().unwrap_or(&vec![]).clone(),
            self.ports.as_ref().unwrap_or(&vec![]).clone(),
        )
        .await?;
        info!("Container \"{}\" created.", &self.name);
        Ok(())
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        docker::commands::container_remove(&self.name, true).await?;
        info!("Container \"{}\" removed.", &self.name);
        Ok(())
    }
}
