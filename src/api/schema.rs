use serde::{Deserialize};


#[derive(Deserialize)]
pub struct CommandRunRequest {
    pub command: String,
    pub args: Vec<String>,
}
