use serde::Deserialize;

use crate::docker;
use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct DockerContainerStart {
    description: Option<String>,
    name: String,
    timeout: Option<i64>,
}

impl DockerContainerStart {
    pub async fn run(&self, _: String) -> Result<(), CommandError> {
        docker::commands::container_start(&self.name).await?;
        Ok(())
    }

    pub async fn revert(&self, _: String) -> Result<(), CommandError> {
        docker::commands::container_stop(
            &self.name,
            self.timeout.unwrap_or(docker::models::DEFAULT_TIMEOUT),
        )
        .await?;
        Ok(())
    }
}
