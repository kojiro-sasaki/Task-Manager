mod models;

use clap::{Parser, Subcommand};
use models::TaskManager;

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Simple Task Manager CLI", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Add { title: String },
    Delete { id: u32 },
    List,
    Search { keyword: String },
    Complete { id: u32 },
}
fn main() {
    let cli = CLI::parse();
    let mut manager = TaskManager::load_from_file().unwrap_or_else(|_| TaskManager::new());
    match cli.command {
        Command::Add { title } => {
            manager.add_task(title).unwrap();
            println!("Task added.");
        }
        Command::Delete { id } => {
            manager.delete_task(id).unwrap();
            println!("Task deleted.");
        }
        Command::Search { keyword } => {
            let result = manager.search_task(&keyword);
            if result.is_empty() {
                println!("Nothing found.")
            } else {
                for t in result {
                    let status = if t.completed { "x" } else { " " };
                    println!("[{}] {} | {}", status, t.id, t.title);
                }
            }
        }
        Command::List => {
            manager.list_tasks();
        }
        Command::Complete { id } => {
            manager.complete_task(id).unwrap();
            println!("Task completed.");
        }
    }
}
