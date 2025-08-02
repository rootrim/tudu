use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    is_done: bool,
    pub id: u32,
    pub name: String,
    pub task_type: TaskType,
    pub tags: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    Basic,
    Counter(i8, i8),
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
    pub fn new_counter(id: u32, name: String, count: i8, tags: Vec<String>) -> Self {
        Self {
            id,
            name,
            is_done: false,
            task_type: TaskType::Counter(0, count),
            tags: tags.into_iter().collect(),
        }
    }
    pub fn is_done(&self) -> bool {
        if self.is_done {
            true
        } else {
            match &self.task_type {
                TaskType::Basic => false,
                TaskType::Counter(current, max) => *current >= *max,
                TaskType::Timed(duration) => duration.is_some() && duration.unwrap().as_secs() == 0,
            }
        }
    }
    pub fn check(&mut self) {
        if self.is_done {
            self.is_done = false;
            return;
        }
        self.is_done = true;
    }
    pub fn add_to_counter(&mut self, count: i8) {
        if let TaskType::Counter(ref mut current, max) = self.task_type {
            if *current < max {
                *current += count;
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
