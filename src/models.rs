use std::fs;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

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
    pub fn add_task(&mut self, title: String) -> Result<(), String> {
        let id = self.next_id;
        let task = Task {
            id,
            title,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;

        self.save_to_file()?;
        Ok(())
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
                self.save_to_file()?;
                return Ok(());
            }
        }
        Err("Task not found".to_string())
    }
    pub fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            self.save_to_file()?;
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
                self.save_to_file()?;
                return Ok(());
            }
        }
        Err("Cannot find task".to_string())
    }
    pub fn save_to_file(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.tasks).map_err(|e| e.to_string())?;
        fs::write("tasks.json", json).map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn load_from_file() -> Result<Self, String> {
        let data = fs::read_to_string("tasks.json").map_err(|e| e.to_string())?;
        let tasks: Vec<Task> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Ok(Self { tasks, next_id })
    }

    pub fn search_task(&self, keyword: &str) -> Vec<&Task> {
        let keyword_lower = keyword.to_lowercase();

        let mut results: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|t| t.title.to_lowercase().contains(&keyword_lower))
            .collect();

        results.sort_by_key(|t| t.id);

        results
    }
}
