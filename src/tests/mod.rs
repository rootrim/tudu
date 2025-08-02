use super::*;

#[test]
fn test_task_creation() {
    let task = Task::new(1, "Example Task".into(), TaskType::Basic);
    assert_eq!(task.id, 1);
    assert_eq!(task.name, "Example Task");
    assert!(!task.is_done);
    assert_eq!(task.task_type, TaskType::Basic);
}

#[test]
fn test_task_check() {
    let mut task = Task::new(2, "Check Task".into(), TaskType::Basic);
    assert!(!task.is_done);
    task.check();
    assert!(task.is_done);
}

#[test]
fn test_task_tags() {
    let mut task = Task::new(3, "Tag Test".into(), TaskType::Basic);
    assert!(task.tags.is_empty());
    task.add_tag("Dio".into());
    task.add_tag("Jotaro".into());
    assert_eq!(task.tags.len(), 2);
    assert!(task.tags.contains(&"Dio".into()));
    task.remove_tag("Dio".into());
    assert!(!task.tags.contains(&"Dio".into()));
}
