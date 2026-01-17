use crate::types::{Todo, TodoList};
use std::fs;

pub struct App {
    pub todos: TodoList,
    should_exit: bool,
}

impl App {
    pub fn create() -> Self {
        App {
            todos: Self::load_todos(),
            should_exit: false,
        }
    }

    fn load_todos() -> TodoList {
        let data = fs::read_to_string("./todos.json");
        match data {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => TodoList::default(),
        }
    }

    pub fn save_todos(&self) {
        let content = serde_json::to_string_pretty(&self.todos).unwrap();
        fs::write("./todos.json", content).expect("Unable to write file");
    }

    pub fn add_todo(&mut self, title: &str) {
        self.todos.push(Todo::new(title));
    }
}
