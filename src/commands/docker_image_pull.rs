use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerImagePull {
    description: String,
    name: String,
    version: String,
}

impl DockerImagePull {
    fn image(&self) -> String {
        format!("{}:{}", self.name, &self.version)
    }

    pub async fn run(&self) -> Result<(), CommandError> {
        match docker::commands::image_pull(&self.image()).await {
            Err(error) => Err(CommandError {
                message: error.message,
            }),
            Ok(_) => {
                println!("\tImage \"{}\" pull complete.", self.image());
                Ok(())
            }
        }
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        match docker::commands::image_remove(&self.image(), true).await {
            Err(error) => Err(CommandError {
                message: error.message,
            }),
            Ok(_) => {
                println!("\tImage \"{}\" removed.", self.image());
                Ok(())
            }
        }
    }
}
