use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use regex::{NoExpand, Regex};
use serde::Deserialize;
use serde_yaml::Value;

use crate::commands::Command;
use crate::errors::AppConfigError;

#[derive(Deserialize, Debug)]
pub struct UnprocessedAppConfiguration {
    pub tasks: HashMap<String, Vec<Value>>,
}

#[derive(Deserialize, Debug)]
pub struct AppConfiguration {
    pub tasks: HashMap<String, Vec<Command>>,
}

pub async fn read(
    app: &str,
    kwargs: HashMap<String, String>,
) -> Result<AppConfiguration, Box<dyn Error>> {
    let app_config = read_config(&app, kwargs).await?;
    let processed_app_config = preprocess_commands(app_config).await?;
    Ok(processed_app_config)
}

async fn read_config(
    name: &str,
    kwargs: HashMap<String, String>,
) -> Result<UnprocessedAppConfiguration, Box<dyn Error>> {
    let config_file = format!("config/apps/{}.yaml", name);
    let mut file = File::open(config_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let contents = insert_variables(&contents, kwargs).await?;

    let app_config: UnprocessedAppConfiguration = serde_yaml::from_str(&contents)?;
    Ok(app_config)
}

async fn insert_variables(
    config_contents: &str,
    kwargs: HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
    let regex = Regex::new(r"<<[a-z_0-9\s]+>>")?;
    let mut result = config_contents.to_string();
    let mut processed: Vec<String> = Vec::new();

    // Replace each found variable with value
    for var in regex.captures_iter(config_contents) {
        let var_name = &var[0]
            .strip_prefix("<<")
            .unwrap()
            .strip_suffix(">>")
            .unwrap()
            .trim();

        // Skip already processed
        if processed.contains(&var_name.to_string()) {
            continue;
        }

        // Find value corresponding to var_name
        let value = kwargs[&var_name.to_string()].clone();
        result = regex.replace_all(&result, NoExpand(&value)).to_string();
        processed.push(var_name.to_string());
    }
    Ok(result)
}

async fn preprocess_commands(
    app_config: UnprocessedAppConfiguration,
) -> Result<AppConfiguration, Box<dyn Error>> {
    let mut processed_app_config = AppConfiguration {
        tasks: HashMap::new(),
    };

    // Manage Task inheritance
    for (task_key, commands) in &app_config.tasks {
        for command in commands {
            if !command.is_string() {
                // Normal commands, convert and assign
                let command_values = app_config.tasks[task_key].clone();
                processed_app_config.tasks.insert(
                    task_key.to_string(),
                    convert_commands(command_values, None)?,
                );
            } else {
                // Task found, replace with associated commands
                let replaceable_task = command.as_str().unwrap_or_default();
                if app_config.tasks.contains_key(replaceable_task) {
                    let command_values = app_config.tasks[replaceable_task].clone();

                    let converted = convert_commands(command_values, Some(&app_config))?;
                    if !processed_app_config.tasks.contains_key(task_key) {
                        // First command to be added
                        processed_app_config
                            .tasks
                            .insert(task_key.to_string(), converted);
                    } else {
                        // Key exists, add to existing commands
                        processed_app_config
                            .tasks
                            .get_mut(task_key)
                            .unwrap_or(&mut Vec::new())
                            .extend(converted);
                    }
                } else {
                    // Key to replace not found!
                    return Err(Box::new(AppConfigError {
                        message: format!(
                            "Cannot parse config, task key to replace not found: \"{}\"",
                            replaceable_task
                        ),
                    }));
                }
            }
        }
    }
    Ok(processed_app_config)
}

fn convert_commands(
    command_values: Vec<Value>,
    app_config: Option<&UnprocessedAppConfiguration>,
) -> Result<Vec<Command>, Box<dyn Error>> {
    let mut found_commands: Vec<Command> = Vec::new();
    for command_value in command_values {
        if command_value.is_string() {
            let new_command_values = replace_command(command_value, app_config.unwrap())?;
            let converted = convert_commands(new_command_values, app_config)?;
            found_commands.extend(converted);
        } else {
            found_commands.push(serde_yaml::from_value(command_value)?);
        }
    }
    Ok(found_commands)
}

fn replace_command(
    command: Value,
    app_config: &UnprocessedAppConfiguration,
) -> Result<Vec<Value>, Box<dyn Error>> {
    let replaceable_task = command.as_str().unwrap_or_default();
    let command_values = app_config.tasks[replaceable_task].clone();
    Ok(command_values)
}
