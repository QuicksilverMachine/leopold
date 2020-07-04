mod docker_image_list;
mod docker_image_pull;
mod docker_container_list;
mod docker_engine_version;

use serde::{Deserialize};
use crate::errors::CommandError;


#[derive(Clone, Deserialize, Debug)]
#[serde(tag = "id")]
pub enum Command {
    DockerImagePull { command: docker_image_pull::DockerImagePull },
    DockerImageList { command: docker_image_list::DockerImageList },
    DockerContainerList { command: docker_container_list::DockerContainerList },
    DockerEngineVersion { command: docker_engine_version::DockerEngineVersion },
}

pub async fn execute_command(command_id: &Command) -> Result<(), CommandError> {
    match command_id {
        Command::DockerImagePull{ command} => Ok(command.execute().await?),
        Command::DockerImageList{ command} => Ok(command.execute().await?),
        Command::DockerContainerList{ command} => Ok(command.execute().await?),
        Command::DockerEngineVersion{ command} => Ok(command.execute().await?),
    }
}

pub async fn revert_command(command_id: &Command) -> Result<(), CommandError> {
    match command_id {
        Command::DockerImagePull{ command} => Ok(command.revert().await?),
        Command::DockerImageList{ command} => Ok(command.revert().await?),
        Command::DockerContainerList{ command} => Ok(command.revert().await?),
        Command::DockerEngineVersion{ command} => Ok(command.revert().await?),
    }
}
