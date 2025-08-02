use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub is_done: bool,
    pub task_type: TaskType,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    Basic,
    Counter(u32),
    Timed(#[serde(with = "humantime_serde")] Option<Duration>),
}

impl Task {
    pub fn new(id: u32, name: String, task_type: TaskType) -> Self {
        Self {
            id,
            name,
            is_done: false,
            task_type,
            tags: Vec::new(),
        }
    }
    pub fn check(&mut self) {
        self.is_done = true;
    }
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
    pub fn remove_tag(&mut self, tag: String) {
        if self.tags.contains(&tag) {
            self.tags.retain(|t| t != &tag);
        }
    }
}
