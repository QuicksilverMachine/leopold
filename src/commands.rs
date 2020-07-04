mod docker_image_list;
mod docker_image_pull;
mod docker_container_list;
mod docker_engine_version;

use serde::{Deserialize};


#[derive(Deserialize, Debug)]
#[serde(tag = "id")]
pub enum Command {
    DockerImagePull { command: docker_image_pull::DockerImagePull },
    DockerImageList { command: docker_image_list::DockerImageList },
    DockerContainerList { command: docker_container_list::DockerContainerList },
    DockerEngineVersion { command: docker_engine_version::DockerEngineVersion },
}

pub async fn execute_command(command_id: &Command) {
    match command_id {
        Command::DockerImagePull{command} => command.execute().await,
        Command::DockerImageList{command} => command.execute().await,
        Command::DockerContainerList{command} => command.execute().await,
        Command::DockerEngineVersion{command} => command.execute().await,
    }
}
