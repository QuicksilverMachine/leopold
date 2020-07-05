use crate::commands::Command;

#[derive(Debug)]
pub struct TaskError {
    pub message: String,
    pub completed_tasks: Vec<Command>
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
        DockerError{message: error.to_string()}
    }
}
