use crate::configuration::parse_configuration;
use crate::commands::execute_command;


pub async fn execute_task(app: &str, task_id: &str) {
    let configuration = parse_configuration(app).await;
    println!("Executing task: {}", task_id);
    for command_id in &configuration.tasks[task_id] {
        execute_command(command_id).await;
    }
}
