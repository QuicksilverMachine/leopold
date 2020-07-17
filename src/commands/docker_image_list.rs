use serde::Deserialize;

use crate::errors::CommandError;
use crate::{docker, logger};

#[derive(Clone, Deserialize, Debug)]
pub struct DockerImageList {
    description: Option<String>,
}

impl DockerImageList {
    pub async fn run(&self, task_id: String) -> Result<(), CommandError> {
        let images = docker::commands::image_list().await?;
        for image in images {
            logger::info_task(task_id.clone(), format!("{}:{}", image.name, image.tag))
        }
        Ok(())
    }

    pub async fn revert(&self, _: String) -> Result<(), CommandError> {
        Ok(())
    }
}
