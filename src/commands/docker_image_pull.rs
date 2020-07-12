use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerImagePull {
    description: Option<String>,
    name: String,
    version: String,
}

impl DockerImagePull {
    fn image(&self) -> String {
        format!("{}:{}", self.name, &self.version)
    }

    pub async fn run(&self) -> Result<(), CommandError> {
        docker::commands::image_pull(&self.image()).await?;
        info!("Image \"{}\" pull complete.", self.image());
        Ok(())
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        docker::commands::image_remove(&self.image(), true).await?;
        info!("Image \"{}\" removed.", self.image());
        Ok(())
    }
}
