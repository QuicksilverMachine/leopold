use futures::FutureExt;

use crate::commands::{revert_command, run_command, Command};
use crate::configuration::parse_configuration;
use crate::errors::TaskError;

pub async fn run(app: &str, task_id: &str) {
    let future = { run_task(app.to_string(), task_id.to_string()).boxed_local() };
    actix::Arbiter::spawn(future);
}

pub async fn run_task(app: String, task_id: String) {
    let configuration = parse_configuration(&app).await;
    println!("Executing task: {}", task_id);

    match run_task_commands(&configuration.tasks[&task_id]).await {
        Err(error) => {
            if error.completed_tasks.is_empty() {
                eprintln!("Task failed: \"{}\".", task_id);
            } else {
                eprintln!("Task failed: \"{}\", attempting revert.", task_id);
            }
            match revert_task_commands(&error.completed_tasks).await {
                Err(error) => {
                    eprintln!(
                        "Failed to revert task \"{}\" due to error: {}.",
                        task_id, error.message
                    );
                }
                _ => {
                    println!("Task reverted: \"{}\"", task_id);
                }
            };
        }
        _ => {
            println!("Task completed: \"{}\"", task_id);
        }
    };
}

async fn run_task_commands(commands: &[Command]) -> Result<(), TaskError> {
    let mut completed: Vec<Command> = Vec::new();
    for command in commands {
        match run_command(command).await {
            Err(error) => {
                eprintln!("Command failed: \"{:?}\".", command);
                Err(TaskError {
                    message: error.message,
                    completed_tasks: completed.clone(),
                })
            }
            Ok(_) => {
                completed.push(command.clone());
                println!("Command completed: \"{:?}\".", command);
                Ok(())
            }
        }?;
    }
    Ok(())
}

async fn revert_task_commands(commands: &[Command]) -> Result<(), TaskError> {
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
