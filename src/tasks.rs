use futures::{FutureExt};

use crate::configuration::parse_configuration;
use crate::commands::{Command, execute_command, revert_command};
use crate::errors::TaskError;


pub async fn execute(app: &str, task_id: &str) {
    let future = {
        execute_task(app.to_string(), task_id.to_string()).boxed_local()
    };
    actix::Arbiter::spawn(future);
}

pub async fn execute_task(app: String, task_id: String) {
    let configuration = parse_configuration(&app).await;
    println!("Executing task: {}", task_id);

    match execute_task_commands(&configuration.tasks[&task_id]).await {
        Err(error) => {
            if error.completed_tasks.is_empty() {
                eprintln!("Task failed: \"{}\".", task_id);
            } else {
                eprintln!("Task failed: \"{}\", attempting revert.", task_id);
            }
            match revert_task_commands(&error.completed_tasks).await {
                Err(error) => {
                    eprintln!("Failed to revert task \"{}\" due to error: {}.", task_id, error.message);
                },
                _ => {
                    println!("Task reverted: \"{}\"", task_id);
                }
            };
        },
        _ => {
            println!("Task completed: \"{}\"", task_id);
        }
    };
}

async fn execute_task_commands(commands: &[Command]) -> Result<(), TaskError> {
    let mut completed: Vec<Command> = Vec::new();
    for command in commands {
        match execute_command(command).await {
            Err(error) => {
                eprintln!("Command failed: \"{:?}\".", command);
                Err(TaskError{message: error.message, completed_tasks: completed.clone()})
            },
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
            Err(error) => Err(
                TaskError{message: error.message, completed_tasks: Vec::new()}
            ),
            _ => Ok(())
        }?;
    }
    Ok(())
}
