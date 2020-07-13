use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerStop {
    description: Option<String>,
    name: String,
    timeout: Option<i64>,
}

static DEFAULT_TIMEOUT: i64 = 10;

impl DockerContainerStop {
    pub async fn run(&self, _: String) -> Result<(), CommandError> {
        docker::commands::container_stop(&self.name, self.timeout.unwrap_or(DEFAULT_TIMEOUT))
            .await?;
        Ok(())
    }

    pub async fn revert(&self, _: String) -> Result<(), CommandError> {
        Ok(())
    }
}
