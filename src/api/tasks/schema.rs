use serde::{Deserialize};


#[derive(Deserialize)]
pub struct TaskExecuteRequest {
    pub app: String,
    pub task_id: String,
}
