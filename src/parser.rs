use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use serde_yaml::{Value};
use serde::{Deserialize};


#[derive(Deserialize, Debug)]
struct DockerImagePullParameters {
    description: String,
    args: Vec<String>,
    kwargs1: Value,
}

#[derive(Deserialize, Debug)]
struct DockerImageListParameters {
    description: String,
    args: Vec<String>,
    kwargs2: Value,
}

#[derive(Deserialize, Debug)]
struct DockerEngineVersionParameters {
    description: String,
    args: Vec<String>,
    kwargs3: Value,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "command")]
enum Command {
    DockerImagePull { parameters: DockerImagePullParameters },
    DockerImageList { parameters: DockerImageListParameters },
    DockerEngineVersion { parameters: DockerEngineVersionParameters },
}

type Task = Vec<Command>;

#[derive(Deserialize, Debug)]
struct Config {
    extends: Vec<String>,
    tasks: HashMap<String,Task>,
}

pub async fn parse() {
    let mut file = File::open("config/app.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let configuration: Config = serde_yaml::from_str(&contents).unwrap();

    for selected_task in  configuration.tasks.keys() {
        println!("Now processing task: {}", selected_task);
        for selected_command in &configuration.tasks[selected_task] {
            match selected_command {
                Command::DockerImagePull{ parameters: _ } => println!("DockerImagePull"),
                Command::DockerImageList{ parameters: _ } => println!("DockerImageList"),
                Command::DockerEngineVersion{ parameters: _ } => println!("DockerEngineVersion"),
            }
        }
    }
}
