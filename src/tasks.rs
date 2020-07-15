use crate::commands::{revert_command, run_command, Command};
use crate::configuration;
use crate::errors::TaskError;

use crate::logger;

pub async fn run(app: &str, task_key: &str, task_id: &str) {
    let future = run_task(app.to_string(), task_key.to_string(), task_id.to_string());
    tokio::spawn(future);
}

pub async fn run_task(app: String, task_key: String, task_id: String) {
    let configuration = match configuration::read(&app).await {
        Ok(configuration) => configuration,
        Err(error) => {
            logger::task_error(
                task_id,
                format!("Error occurred while fetching app configuration: {}", error),
            );
            return;
        }
    };

    // Check if requested task exists for selected app
    if !&configuration.tasks.contains_key(&task_key) {
        logger::task_error(
            task_id,
            format!("Task not found: \"{}\" for app \"{}\"", task_key, app),
        );
        return;
    }

    logger::info("Running task");
    match run_commands(&configuration.tasks[&task_key], task_id.clone()).await {
        Err(error) => {
            if error.completed_tasks.is_empty() {
                logger::task_error(&task_id, "Task failed");
            } else {
                logger::task_error(&task_id, "Task failed, attempting revert");
            }
            match revert_commands(&error.completed_tasks, task_id.clone()).await {
                Err(error) => {
                    logger::task_error(
                        &task_id,
                        format!(
                            "Task revert failed for {} due to error: {}",
                            task_key, error.message
                        ),
                    );
                }
                _ => {
                    logger::task_info(&task_id, "Task reverted");
                }
            };
        }
        _ => {
            logger::task_info(task_id, "Task completed");
        }
    };
}

async fn run_commands(commands: &[Command], task_id: String) -> Result<(), TaskError> {
    let mut completed: Vec<Command> = Vec::new();
    for command in commands {
        logger::task_info(&task_id, format!("Running command: {}", command));
        match run_command(command, task_id.clone()).await {
            Err(error) => {
                logger::task_error(&task_id, format!("Command failed: {}", command));
                Err(TaskError {
                    message: error.message,
                    completed_tasks: completed.clone(),
                })
            }
            Ok(_) => {
                completed.push(command.clone());
                logger::task_info(&task_id, format!("Command completed: {}", command));
                Ok(())
            }
        }?;
    }
    Ok(())
}

async fn revert_commands(commands: &[Command], task_id: String) -> Result<(), TaskError> {
    for command in commands.iter().rev() {
        match revert_command(command, task_id.clone()).await {
            Err(error) => Err(TaskError {
                message: error.message,
                completed_tasks: Vec::new(),
            }),
            _ => Ok(()),
        }?;
    }
    Ok(())
}
