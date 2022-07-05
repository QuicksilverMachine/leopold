use std::error::Error;
use std::fmt::{Display, Formatter};


use crate::commands::Command;

#[derive(Debug)]
pub struct AppConfigError {
    pub message: String,
}

impl Error for AppConfigError {}

impl Display for AppConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub struct TaskError {
    pub message: String,
    pub completed_tasks: Vec<Command>,
}

#[derive(Debug)]
pub struct CommandError {
    pub message: String,
}

#[derive(Debug)]
pub struct DockerError {
    pub message: String,
}

impl From<bollard::errors::Error> for DockerError {
    fn from(error: bollard::errors::Error) -> Self {
        DockerError {
            message: error.to_string(),
        }
    }
}

impl From<DockerError> for CommandError {
    fn from(error: DockerError) -> Self {
        CommandError {
            message: error.message,
        }
    }
}
