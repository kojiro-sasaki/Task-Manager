mod models;
use models::TaskManager;

fn main() {
    let mut manager = TaskManager::new();

    manager.add_task("Learn Rust".to_string());
    manager.add_task("Build project".to_string());

    manager.list_tasks();

    manager.complete_task(1).unwrap();

    manager.delete_task(2).unwrap();

    manager.list_tasks();
    manager.update_task(1, "Goon".to_string());
    manager.list_tasks();
}
