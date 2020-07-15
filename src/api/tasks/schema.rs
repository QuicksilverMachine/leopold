use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TaskRunRequest {
    pub app: String,
    pub task_key: String,
}

#[derive(Serialize)]
pub struct TaskRunResponse {
    pub task_id: String,
}
