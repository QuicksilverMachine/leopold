use crate::docker;
use crate::errors::CommandError;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerList {
    description: String,
}

impl DockerContainerList {
    pub async fn run(&self) -> Result<(), CommandError> {
        let container_list = docker::container_list().await;
        match container_list {
            Err(error) => Err(CommandError {
                message: error.message,
            }),
            Ok(containers) => {
                for container in containers {
                    println!("{}", container.name)
                }
                Ok(())
            }
        }
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}
