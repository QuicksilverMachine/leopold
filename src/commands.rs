use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

mod docker_image_list;
mod docker_image_pull;
mod docker_container_list;
mod docker_engine_version;

use docker_image_list::DockerImageList;
use docker_image_pull::DockerImagePull;
use docker_container_list::DockerContainerList;
use docker_engine_version::DockerEngineVersion;

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "id")]
pub enum Command {
    DockerImagePull { command: DockerImagePull },
    DockerImageList { command: DockerImageList },
    DockerContainerList { command: DockerContainerList },
    DockerEngineVersion { command: DockerEngineVersion },
}

type Task = Vec<Command>;

#[derive(Deserialize, Debug)]
struct Configuration {
    extends: Vec<String>,
    tasks: HashMap<String,Task>,
}

async fn parse_configuration(app: &String) -> Configuration {
    let config_file = format!("config/{}.yaml", app);
    let mut file = File::open(config_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let configuration: Configuration = serde_yaml::from_str(&contents).unwrap();
    configuration
}

pub async fn execute_task(app: &String, task_id: &String) {
    let configuration = parse_configuration(app).await;
    println!("Executing task: {}", task_id);
    for command_id in &configuration.tasks[task_id] {
        execute_command(command_id).await;
    }
}

async fn execute_command(command_id: &Command) {
    match command_id {
        Command::DockerImagePull{command} => command.execute().await,
        Command::DockerImageList{command} => command.execute().await,
        Command::DockerContainerList{command} => command.execute().await,
        Command::DockerEngineVersion{command} => command.execute().await,
    }
}
