use std::time::Duration;

use serde::Deserialize;

use crate::errors::CommandError;

#[derive(Clone, Deserialize, Debug)]
pub struct Sleep {
    description: Option<String>,
    duration: f64,
}

impl Sleep {
    pub async fn run(&self, _: String) -> Result<(), CommandError> {
        tokio::time::sleep(Duration::from_secs_f64(self.duration)).await;
        Ok(())
    }

    pub async fn revert(&self, _: String) -> Result<(), CommandError> {
        Ok(())
    }
}
