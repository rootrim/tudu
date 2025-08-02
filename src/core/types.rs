use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub is_done: bool,
    pub task_type: TaskType,
    pub tags: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    Basic,
    Counter(u32),
    Timed(#[serde(with = "humantime_serde")] Option<Duration>),
}

impl Task {
    pub fn new_basic(id: u32, name: String, tags: Vec<String>) -> Self {
        Self {
            id,
            name,
            is_done: false,
            task_type: TaskType::Basic,
            tags: tags.into_iter().collect(),
        }
    }
    pub fn check(&mut self) {
        self.is_done = true;
    }
    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }
    pub fn remove_tag(&mut self, tag: String) {
        self.tags.remove(&tag);
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
}
