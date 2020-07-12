mod docker_container_create;
mod docker_container_list;
mod docker_container_remove;
mod docker_container_restart;
mod docker_container_start;
mod docker_container_stop;
mod docker_image_list;
mod docker_image_pull;
mod docker_version;

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
    DockerContainerCreate {
        command: docker_container_create::DockerContainerCreate,
    },
    DockerContainerRemove {
        command: docker_container_remove::DockerContainerRemove,
    },
    DockerContainerStart {
        command: docker_container_start::DockerContainerStart,
    },
    DockerContainerStop {
        command: docker_container_stop::DockerContainerStop,
    },
    DockerContainerRestart {
        command: docker_container_restart::DockerContainerRestart,
    },
    DockerVersion {
        command: docker_version::DockerVersion,
    },
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self {
            Command::DockerImagePull { command: _ } => write!(f, "DockerImagePull"),
            Command::DockerImageList { command: _ } => write!(f, "DockerImageList"),
            Command::DockerContainerList { command: _ } => write!(f, "DockerContainerList"),
            Command::DockerContainerCreate { command: _ } => write!(f, "DockerContainerCreate"),
            Command::DockerContainerRemove { command: _ } => write!(f, "DockerContainerRemove"),
            Command::DockerContainerStart { command: _ } => write!(f, "DockerContainerStart"),
            Command::DockerContainerStop { command: _ } => write!(f, "DockerContainerStop"),
            Command::DockerContainerRestart { command: _ } => write!(f, "DockerContainerRestart"),
            Command::DockerVersion { command: _ } => write!(f, "DockerVersion"),
        }
    }
}

pub async fn run_command(command_container: &Command) -> Result<(), CommandError> {
    match command_container {
        Command::DockerImagePull { command } => Ok(command.run().await?),
        Command::DockerImageList { command } => Ok(command.run().await?),
        Command::DockerContainerList { command } => Ok(command.run().await?),
        Command::DockerContainerCreate { command } => Ok(command.run().await?),
        Command::DockerContainerRemove { command } => Ok(command.run().await?),
        Command::DockerContainerStart { command } => Ok(command.run().await?),
        Command::DockerContainerStop { command } => Ok(command.run().await?),
        Command::DockerContainerRestart { command } => Ok(command.run().await?),
        Command::DockerVersion { command } => Ok(command.run().await?),
    }
}

pub async fn revert_command(command_container: &Command) -> Result<(), CommandError> {
    match command_container {
        Command::DockerImagePull { command } => Ok(command.revert().await?),
        Command::DockerImageList { command } => Ok(command.revert().await?),
        Command::DockerContainerList { command } => Ok(command.revert().await?),
        Command::DockerContainerCreate { command } => Ok(command.revert().await?),
        Command::DockerContainerRemove { command } => Ok(command.revert().await?),
        Command::DockerContainerStart { command } => Ok(command.revert().await?),
        Command::DockerContainerStop { command } => Ok(command.revert().await?),
        Command::DockerContainerRestart { command } => Ok(command.revert().await?),
        Command::DockerVersion { command } => Ok(command.revert().await?),
    }
}
