use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerRemove {
    name: String,
    force: Option<bool>,
}

impl DockerContainerRemove {
    pub async fn run(&self) -> Result<(), CommandError> {
        match docker::commands::container_remove(&self.name, self.force.unwrap_or(true)).await {
            Err(error) => Err(CommandError {
                message: error.message,
            }),
            Ok(_) => {
                println!("\tContainer \"{}\" removed.", &self.name);
                Ok(())
            }
        }
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}