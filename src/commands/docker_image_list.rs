use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerImageList {
    description: Option<String>,
}

impl DockerImageList {
    pub async fn run(&self) -> Result<(), CommandError> {
        let images = docker::commands::image_list().await?;
        for image in images {
            info!("{}:{}", image.name, image.tag)
        }
        Ok(())
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}
