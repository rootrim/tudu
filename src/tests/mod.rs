use super::*;
use crate::core::types::TaskType;

#[test]
fn test_task_creation() {
    let task = Task::new_basic(1, "Example Task".into(), vec![]);
    assert_eq!(task.id, 1);
    assert_eq!(task.name, "Example Task");
    assert!(!task.is_done);
    assert_eq!(task.task_type, TaskType::Basic);
}

#[test]
fn test_task_check() {
    let mut task = Task::new_basic(2, "Check Task".into(), vec![]);
    assert!(!task.is_done);
    task.check();
    assert!(task.is_done);
}

#[test]
fn test_task_tags() {
    let mut task = Task::new_basic(3, "Tag Test".into(), vec![]);
    assert!(task.tags.is_empty());
    task.add_tag("Dio".into());
    task.add_tag("Jotaro".into());
    task.add_tag("Dio".into());
    assert_eq!(task.tags.len(), 2);
    assert!(task.has_tag("Dio"));
    task.remove_tag("Dio".into());
    assert!(!task.has_tag("Dio"));
}
