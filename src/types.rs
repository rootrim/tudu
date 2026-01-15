use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    created_at: u64,
    pub title: String,
    completed: bool,
}

impl Todo {
    pub fn new(title: &str) -> Self {
        Todo {
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            title: title.to_string(),
            completed: false,
        }
    }

    pub fn toggle_mark(&mut self) {
        self.completed = !self.completed;
    }

    pub fn pretty_print(&self) -> String {
        format!(
            "[{}] {} (created at: {})",
            if self.completed { "x" } else { " " },
            self.title,
            self.created_at
        )
    }
}
