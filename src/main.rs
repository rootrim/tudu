use crate::core::types::{Task, TaskType};

pub mod core;
#[cfg(test)]
mod tests;

fn main() {
    let task = Task {
        id: 1,
        name: "Example Task".to_string(),
        is_done: false,
        task_type: TaskType::Basic,
        tags: vec!["example".to_string(), "task".to_string()],
    };
    println!("Goodbye, JoJo!");
    println!("Task created: {task:?}");
}
