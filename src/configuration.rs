use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;
use serde_yaml::Value;

use crate::commands::Command;
use crate::errors::AppConfigError;

#[derive(Deserialize, Debug)]
pub struct AppConfiguration {
    pub extends: Vec<String>,
    pub tasks: HashMap<String, Vec<Value>>,
}

#[derive(Deserialize, Debug)]
pub struct ProcessedAppConfiguration {
    pub extends: Vec<String>,
    pub tasks: HashMap<String, Vec<Command>>,
}

pub async fn app_configuration(app: &str) -> Result<ProcessedAppConfiguration, Box<dyn Error>> {
    let config_file = format!("config/apps/{}.yaml", app);
    let mut file = File::open(config_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let app_config: AppConfiguration = serde_yaml::from_str(&contents)?;
    let processed_app_config = preprocess(app_config).await?;
    Ok(processed_app_config)
}

async fn preprocess(
    app_config: AppConfiguration,
) -> Result<ProcessedAppConfiguration, Box<dyn Error>> {
    let mut processed_app_config = ProcessedAppConfiguration {
        extends: app_config.extends,
        tasks: HashMap::new(),
    };

    // TODO: Manage file inheritance

    // Manage Task inheritance
    for (task_id, commands) in &app_config.tasks {
        for command in commands {
            if !command.is_string() {
                // Normal commands, convert and assign
                let command_values = app_config.tasks[task_id].clone();
                processed_app_config.tasks.insert(
                    task_id.to_string(),
                    converted_commands(command_values).await?,
                );
            } else {
                // Task found, replace with associated commands
                let replaceable_task = command.as_str().unwrap_or_default();
                if app_config.tasks.contains_key(replaceable_task) {
                    let command_values = app_config.tasks[replaceable_task].clone();

                    let converted = converted_commands(command_values).await?;
                    if !processed_app_config.tasks.contains_key(task_id) {
                        // First command to be added
                        processed_app_config
                            .tasks
                            .insert(task_id.to_string(), converted);
                    } else {
                        // Key exists, add to existing commands
                        processed_app_config
                            .tasks
                            .get_mut(task_id)
                            .unwrap_or(&mut Vec::new())
                            .extend(converted);
                    }
                } else {
                    // Key to replace not found!
                    return Err(Box::new(AppConfigError {
                        message: format!(
                            "Cannot parse config, task key to replace not found: \"{}\".",
                            replaceable_task
                        ),
                    }));
                }
            }
        }
    }
    let processed_app_config = processed_app_config;
    Ok(processed_app_config)
}

async fn converted_commands(command_values: Vec<Value>) -> Result<Vec<Command>, Box<dyn Error>> {
    let mut found_commands: Vec<Command> = Vec::new();
    for command_value in command_values {
        found_commands.push(serde_yaml::from_value(command_value)?);
    }
    let found_commands = found_commands;
    Ok(found_commands)
}
