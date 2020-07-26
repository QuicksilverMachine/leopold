use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TaskRunRequest {
    pub app: String,
    pub task_key: String,
    pub kwargs: HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
pub struct TaskRunResponse {
    pub task_id: String,
}

pub async fn convert_kwargs_map(
    kwargs: HashMap<String, serde_json::Value>,
) -> HashMap<String, String> {
    let mut string_map = HashMap::new();
    for (k, v) in kwargs {
        if v.is_string() {
            string_map.insert(k, v.as_str().unwrap_or_default().to_string());
        } else {
            string_map.insert(k, v.to_string());
        }
    }
    string_map
}
