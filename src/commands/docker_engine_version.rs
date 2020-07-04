use crate::docker;
use serde::Deserialize;
use crate::errors::CommandError;


#[derive(Clone, Deserialize, Debug)]
pub struct DockerEngineVersion {
    description: String,
}

impl DockerEngineVersion {
    pub async fn execute(&self) ->  Result<(), CommandError> {
        match docker::version().await {
            Err(error) => Err(CommandError{message: error.message}),
            Ok(version) => {
                println!("Docker engine version: {}", version);
                Ok(())
            }
        }
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }}
