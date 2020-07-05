use crate::docker;
use crate::errors::CommandError;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerImageList {
    description: String,
}

impl DockerImageList {
    pub async fn execute(&self) -> Result<(), CommandError> {
        let images = docker::image_list().await;
        match images {
            Err(error) => Err(CommandError {
                message: error.message,
            }),
            Ok(images) => {
                for image in images {
                    println!("{}", image.name)
                }
                Ok(())
            }
        }
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}
