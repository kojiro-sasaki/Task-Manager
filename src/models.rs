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
            println!("{:?}", tasks);
        }
    }
}
