#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub id: u32,
    pub completed: bool,
}
pub struct TaskManager {
    pub tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
    pub fn add_task(&mut self, title: String) {
        let id = self.next_id;
        let task = Task {
            id,
            title,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
    }
    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found");
            return;
        }
        let mut sorted = self.tasks.clone();
        sorted.sort_by_key(|t| t.id);
        for t in &sorted {
            let status = if t.completed { "x" } else { " " };
            println!("[{}]{} | [{}]", status, t.id, t.title);
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
    pub fn update_task(&mut self, id: u32, new_title: String) -> Result<(), String> {
        if new_title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        for t in &mut self.tasks {
            if t.id == id {
                t.title = new_title;
                t.completed = false;
                return Ok(());
            }
        }
        Err("Cannot find task".to_string())
    }
}
