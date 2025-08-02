use crate::core::types::{Task, TaskType};

pub mod core;
#[cfg(test)]
mod tests;

fn main() {
    let task = Task::new(1, "Habibi".into(), TaskType::Basic);
    println!("Goodbye, JoJo!");
    println!("Task created: {task:?}");
}
