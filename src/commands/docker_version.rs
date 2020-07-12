use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerVersion {
    description: Option<String>,
}

impl DockerVersion {
    pub async fn run(&self) -> Result<(), CommandError> {
        let version = docker::commands::version().await?;
        info!("Docker engine version: {}", version);
        Ok(())
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}
