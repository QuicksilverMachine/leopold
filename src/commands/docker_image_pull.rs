use crate::docker;
use serde::Deserialize;
use crate::errors::CommandError;


#[derive(Clone, Deserialize, Debug)]
pub struct DockerImagePull {
    description: String,
    name: String,
    version: String
}

impl DockerImagePull {
    fn image(&self) -> String {
        format!("{}:{}", self.name, &self.version)
    }

    pub async fn execute(&self) -> Result<(), CommandError> {
        match docker::image_pull(&self.image()).await {
            Err(error) => Err(CommandError{message:error.message}),
            Ok(_) => Ok(())
        }
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        match docker::image_remove(&self.image(), true).await {
            Err(error) => Err(CommandError{message: error.message}),
            Ok(_) => {
                println!("Image removed");
                Ok(())
            },
        }
    }
}
