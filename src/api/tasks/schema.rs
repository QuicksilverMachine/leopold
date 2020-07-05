use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TaskRunRequest {
    pub app: String,
    pub task_id: String,
}

#[derive(Serialize)]
pub struct TaskRunResponse {
    pub message: String,
}
