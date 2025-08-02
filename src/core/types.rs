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
    Counter(u8, u8),
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
    pub fn new_counter(id: u32, name: String, count: u8, tags: Vec<String>) -> Self {
        Self {
            id,
            name,
            is_done: false,
            task_type: TaskType::Counter(0, count),
            tags: tags.into_iter().collect(),
        }
    }
    pub fn check(&mut self) {
        if self.is_done {
            self.is_done = false;
            return;
        }
        self.is_done = true;
    }
    pub fn increment_counter(&mut self) {
        if let TaskType::Counter(ref mut current, max) = self.task_type {
            if *current < max {
                *current += 1;
            }
        }
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
