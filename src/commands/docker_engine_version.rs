use crate::docker;
use crate::errors::CommandError;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerEngineVersion {
    description: String,
}

impl DockerEngineVersion {
    pub async fn run(&self) -> Result<(), CommandError> {
        match docker::version().await {
            Err(error) => Err(CommandError {
                message: error.message,
            }),
            Ok(version) => {
                println!("Docker engine version: {}", version);
                Ok(())
            }
        }
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}
