use serde::Deserialize;

use crate::errors::CommandError;
use crate::{docker, logger};

#[derive(Clone, Deserialize, Debug)]
pub struct DockerVersion {
    description: Option<String>,
}

impl DockerVersion {
    pub async fn run(&self, task_id: String) -> Result<(), CommandError> {
        let version = docker::commands::version().await?;
        logger::info_task(task_id.clone(), version);
        Ok(())
    }

    pub async fn revert(&self, _: String) -> Result<(), CommandError> {
        Ok(())
    }
}
