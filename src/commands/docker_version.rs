use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerVersion {
    description: String,
}

impl DockerVersion {
    pub async fn run(&self) -> Result<(), CommandError> {
        match docker::commands::version().await {
            Err(error) => Err(CommandError {
                message: error.message,
            }),
            Ok(version) => {
                println!("\tDocker engine version: {}", version);
                Ok(())
            }
        }
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}
