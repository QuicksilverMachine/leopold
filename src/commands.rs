mod docker_container_list;
mod docker_engine_version;
mod docker_image_list;
mod docker_image_pull;

use std::fmt::{Debug, Display, Formatter};

use serde::Deserialize;

use crate::errors::CommandError;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "id")]
pub enum Command {
    DockerImagePull {
        command: docker_image_pull::DockerImagePull,
    },
    DockerImageList {
        command: docker_image_list::DockerImageList,
    },
    DockerContainerList {
        command: docker_container_list::DockerContainerList,
    },
    DockerEngineVersion {
        command: docker_engine_version::DockerEngineVersion,
    },
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self {
            Command::DockerImagePull { command: _ } => write!(f, "DockerImagePull"),
            Command::DockerImageList { command: _ } => write!(f, "DockerImageList"),
            Command::DockerContainerList { command: _ } => write!(f, "DockerContainerList"),
            Command::DockerEngineVersion { command: _ } => write!(f, "DockerEngineVersion"),
        }
    }
}

pub async fn run_command(command_container: &Command) -> Result<(), CommandError> {
    match command_container {
        Command::DockerImagePull { command } => Ok(command.run().await?),
        Command::DockerImageList { command } => Ok(command.run().await?),
        Command::DockerContainerList { command } => Ok(command.run().await?),
        Command::DockerEngineVersion { command } => Ok(command.run().await?),
    }
}

pub async fn revert_command(command_container: &Command) -> Result<(), CommandError> {
    match command_container {
        Command::DockerImagePull { command } => Ok(command.revert().await?),
        Command::DockerImageList { command } => Ok(command.revert().await?),
        Command::DockerContainerList { command } => Ok(command.revert().await?),
        Command::DockerEngineVersion { command } => Ok(command.revert().await?),
    }
}
