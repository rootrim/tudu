use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    name: String,
    is_done: bool,
    task_type: TaskType,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Basic,
    Counter(u32),
    Timed(#[serde(with = "humantime_serde")] Option<Duration>),
}
