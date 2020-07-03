use serde::{Deserialize};

use super::docker_image_list::DockerImageList;
use super::docker_image_pull::DockerImagePull;
use super::docker_container_list::DockerContainerList;
use super::docker_engine_version::DockerEngineVersion;


#[derive(Deserialize, Debug)]
#[serde(tag = "id")]
pub(crate) enum Command {
    DockerImagePull { command: DockerImagePull },
    DockerImageList { command: DockerImageList },
    DockerContainerList { command: DockerContainerList },
    DockerEngineVersion { command: DockerEngineVersion },
}

pub(crate) async fn execute_command(command_id: &Command) {
    match command_id {
        Command::DockerImagePull{command} => command.execute().await,
        Command::DockerImageList{command} => command.execute().await,
        Command::DockerContainerList{command} => command.execute().await,
        Command::DockerEngineVersion{command} => command.execute().await,
    }
}
