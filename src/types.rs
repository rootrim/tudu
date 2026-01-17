use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    items: Vec<Todo>,
    state: ListState,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    created_at: u64,
    pub title: String,
    todo_state: TodoState,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TodoState {
    Completed,
    Todo,
}

impl Todo {
    pub fn new(title: &str) -> Self {
        Todo {
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            title: title.to_string(),
            todo_state: TodoState::Todo,
        }
    }

    pub fn toggle_mark(&mut self) {
        match self.todo_state {
            TodoState::Completed => self.todo_state = TodoState::Todo,
            TodoState::Todo => self.todo_state = TodoState::Completed,
        }
    }

    pub fn pretty(&self) -> String {
        format!(
            "[{}] {} (created at: {})",
            if self.todo_state == TodoState::Completed {
                "x"
            } else {
                " "
            },
            self.title,
            self.created_at
        )
    }
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            items: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn push(&mut self, todo: Todo) {
        self.items.push(todo);
    }

    pub fn items(&self) -> &Vec<Todo> {
        &self.items
    }

    pub fn remove_selected(&mut self) {
        if let Some(selected) = self.state.selected() {
            self.items.remove(selected);
        };
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}
