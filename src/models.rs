#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub id: u32,
    pub completed: bool,
}
pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
    pub fn add_task(&mut self, title: String) {
        let id = self.tasks.len() as u32 + 1;
        let task = Task {
            id,
            title,
            completed: false,
        };
        self.tasks.push(task);
    }
    pub fn list_tasks(&self) {
        for t in &self.tasks {
            let status = if t.completed { "x" } else { " " };
            println!("[{}]{} |[{}]", status, t.id, t.title);
        }
    }
    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        for t in &mut self.tasks {
            if t.id == id {
                t.completed = true;
                return Ok(());
            }
        }
        Err("Task not found".to_string())
    }
    pub fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            Ok(())
        } else {
            Err("Cannot find task".to_string())
        }
    }
}
