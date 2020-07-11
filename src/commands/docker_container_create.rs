use crate::docker;
use crate::errors::CommandError;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerCreate {
    image: String,
    name: String,
    mounts: Vec<String>,
    ports: Vec<docker::DockerContainerPortBinding>,
}

impl DockerContainerCreate {
    pub async fn run(&self) -> Result<(), CommandError> {
        match docker::container_create(
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
        match docker::container_remove(&self.name, true).await {
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
