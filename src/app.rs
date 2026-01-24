use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
};

use crate::types::{Todo, TodoList};
use std::fs;

pub struct App {
    pub todos: TodoList,
    pub is_editing: bool,
    should_exit: bool,
}

impl App {
    pub fn create() -> Self {
        App {
            todos: Self::load_todos(),
            is_editing: false,
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

    pub fn run(mut self, mut terminal: DefaultTerminal) {
        while !self.should_exit {
            terminal
                .draw(|frame| {
                    frame.render_widget(&mut self, frame.area());
                })
                .unwrap();
            if let Event::Key(key) = event::read().unwrap() {
                self.handle_key_event(key);
            }
        }
        self.save_todos();
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        if self.is_editing {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    self.is_editing = false;
                }
                KeyCode::Char(c) => {
                    if let Some(current) = self.todos.selected_mut() {
                        current.title.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if let Some(current) = self.todos.selected_mut() {
                        current.title.pop();
                    }
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char('q') => {
                    self.should_exit = true;
                }
                KeyCode::Char('r') => {
                    if self.todos.state.selected().is_some() {
                        self.is_editing = true;
                    }
                }
                KeyCode::Char('a') => {
                    self.todos.push(Todo::new(""));
                    self.todos.state.select(Some(self.todos.items.len() - 1));
                    self.is_editing = true;
                }
                KeyCode::Char('m') | KeyCode::Char(' ') => {
                    self.todos.toggle_selected();
                }
                KeyCode::Char('d') => {
                    self.todos.remove_selected();
                }
                KeyCode::Char('j') => {
                    self.todos.state.select_next();
                }
                KeyCode::Char('k') => {
                    self.todos.state.select_previous();
                }
                KeyCode::Esc => {
                    self.todos.state.select(None);
                }
                _ => {}
            }
        }
    }
}
