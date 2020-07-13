use tokio;

use crate::commands::{revert_command, run_command, Command};
use crate::configuration;
use crate::errors::TaskError;

pub async fn run(app: &str, task_key: &str) {
    let future = run_task(app.to_string(), task_key.to_string());
    tokio::spawn(future);
}

pub async fn run_task(app: String, task_key: String) {
    let configuration = match configuration::read(&app).await {
        Ok(configuration) => configuration,
        Err(error) => {
            error!(
                "Error occurred while fetching app configuration. {}",
                error.to_string()
            );
            return;
        }
    };

    // Check if requested task exists for selected app
    if !&configuration.tasks.contains_key(&task_key) {
        error!("Task not found: \"{}\" for app \"{}\".", task_key, app);
        return;
    }

    info!("Executing task: \"{}\"", task_key);
    match run_commands(&configuration.tasks[&task_key]).await {
        Err(error) => {
            if error.completed_tasks.is_empty() {
                error!("Task failed: \"{}\".", task_key);
            } else {
                error!("Task failed: \"{}\", attempting revert.", task_key);
            }
            match revert_commands(&error.completed_tasks).await {
                Err(error) => {
                    error!(
                        "Task revert failed for \"{}\" due to error: {}.",
                        task_key, error.message
                    );
                }
                _ => {
                    info!("Task reverted: \"{}\"", task_key);
                }
            };
        }
        _ => {
            info!("Task completed: \"{}\"", task_key);
        }
    };
}

async fn run_commands(commands: &[Command]) -> Result<(), TaskError> {
    let mut completed: Vec<Command> = Vec::new();
    for command in commands {
        info!("Running command: \"{}\"", command.to_string());
        match run_command(command).await {
            Err(error) => {
                error!("Command failed: \"{}\".", command.to_string());
                Err(TaskError {
                    message: error.message,
                    completed_tasks: completed.clone(),
                })
            }
            Ok(_) => {
                completed.push(command.clone());
                info!("Command completed: \"{}\".", command.to_string());
                Ok(())
            }
        }?;
    }
    Ok(())
}

async fn revert_commands(commands: &[Command]) -> Result<(), TaskError> {
    for command in commands.iter().rev() {
        match revert_command(command).await {
            Err(error) => Err(TaskError {
                message: error.message,
                completed_tasks: Vec::new(),
            }),
            _ => Ok(()),
        }?;
    }
    Ok(())
}
