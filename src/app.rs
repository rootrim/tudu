use crate::types::Todo;
use std::fs;

pub struct App {
    pub todos: Vec<Todo>,
}

impl App {
    pub fn create() -> Self {
        App {
            todos: Self::load_todos(),
        }
    }

    fn load_todos() -> Vec<Todo> {
        let data = fs::read_to_string("./todos.json");
        match data {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Vec::new(),
        }
    }

    pub fn save_todos(&self) {
        let content = serde_json::to_string_pretty(&self.todos).unwrap();
        fs::write("./todos.json", content).expect("Unable to write file");
    }
}
