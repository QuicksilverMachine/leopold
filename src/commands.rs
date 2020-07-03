use crate::configuration::parse_configuration;
use crate::commands::command::execute_command;

pub(crate) mod command;
mod docker_image_list;
mod docker_image_pull;
mod docker_container_list;
mod docker_engine_version;


pub async fn execute_task(app: &str, task_id: &str) {
    let configuration = parse_configuration(app).await;
    println!("Executing task: {}", task_id);
    for command_id in &configuration.tasks[task_id] {
        execute_command(command_id).await;
    }
}
