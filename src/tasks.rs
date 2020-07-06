use futures::FutureExt;

use crate::commands::{revert_command, run_command, Command};
use crate::configuration;
use crate::errors::TaskError;

pub async fn run(app: &str, task_id: &str) {
    let future = { run_task(app.to_string(), task_id.to_string()).boxed_local() };
    actix::Arbiter::spawn(future);
}

pub async fn run_task(app: String, task_id: String) {
    let configuration = match configuration::read(&app).await {
        Ok(configuration) => configuration,
        Err(error) => {
            eprintln!(
                "* Error occurred while fetching app configuration. {}",
                error.to_string()
            );
            return;
        }
    };

    // Check if requested task exists for selected app
    if !&configuration.tasks.contains_key(&task_id) {
        eprintln!("> Task not found: \"{}\" for app \"{}\".", task_id, app);
        return;
    }

    println!("> Executing task: \"{}\"", task_id);
    match run_task_commands(&configuration.tasks[&task_id]).await {
        Err(error) => {
            if error.completed_tasks.is_empty() {
                eprintln!("> Task failed: \"{}\".", task_id);
            } else {
                eprintln!("> Task failed: \"{}\", attempting revert.", task_id);
            }
            match revert_task_commands(&error.completed_tasks).await {
                Err(error) => {
                    eprintln!(
                        "> Task revert failed for \"{}\" due to error: {}.",
                        task_id, error.message
                    );
                }
                _ => {
                    println!("> Task reverted: \"{}\"", task_id);
                }
            };
        }
        _ => {
            println!("> Task completed: \"{}\"", task_id);
        }
    };
}

async fn run_task_commands(commands: &[Command]) -> Result<(), TaskError> {
    let mut completed: Vec<Command> = Vec::new();
    for command in commands {
        println!("- Running command: \"{}\"", command.to_string());
        match run_command(command).await {
            Err(error) => {
                eprintln!("- Command failed: \"{}\".", command.to_string());
                Err(TaskError {
                    message: error.message,
                    completed_tasks: completed.clone(),
                })
            }
            Ok(_) => {
                completed.push(command.clone());
                println!("- Command completed: \"{}\".", command.to_string());
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
