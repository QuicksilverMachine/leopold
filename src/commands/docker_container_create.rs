use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerCreate {
    description: Option<String>,
    image: String,
    name: String,
    mounts: Vec<String>,
    ports: Vec<docker::models::DockerContainerPortBinding>,
}

impl DockerContainerCreate {
    pub async fn run(&self) -> Result<(), CommandError> {
        match docker::commands::container_create(
            &self.image,
            &self.name,
            self.mounts.clone(),
            self.ports.clone(),
        )
        .await
        {
            Err(error) => Err(CommandError {
                message: error.message,
            }),
            Ok(_) => {
                println!("\tContainer \"{}\" created.", &self.name);
                Ok(())
            }
        }
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        match docker::commands::container_remove(&self.name, true).await {
            Err(error) => Err(CommandError {
                message: error.message,
            }),
            Ok(_) => {
                println!("\tContainer \"{}\" removed.", &self.name);
                Ok(())
            }
        }
    }
}
