use crate::core::types::Task;

pub mod core;
#[cfg(test)]
mod tests;

fn main() {
    let task = Task::new_basic(1, "Habibi".into(), vec!["Dio".into(), "Jotaro".into()]);
    println!("Goodbye, JoJo!");
    println!("Task created: {task:?}");
}
