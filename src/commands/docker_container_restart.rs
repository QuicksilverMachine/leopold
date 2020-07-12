use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerRestart {
    description: Option<String>,
    name: String,
    timeout: Option<i64>,
}

static DEFAULT_TIMEOUT: i64 = 10;

impl DockerContainerRestart {
    pub async fn run(&self) -> Result<(), CommandError> {
        docker::commands::container_restart(&self.name, self.timeout.unwrap_or(DEFAULT_TIMEOUT))
            .await?;
        Ok(())
    }

    pub async fn revert(&self) -> Result<(), CommandError> {
        Ok(())
    }
}
