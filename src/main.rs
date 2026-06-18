use std::fs;
use std::io::{self, Write};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

const DB_FILE: &str = "tasks.json";

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string(DB_FILE) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(DB_FILE, json).expect("Failed to save tasks");
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    tasks.push(Task { id, description, done: false });
    println!("Added task #{}", id);
}

fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }
    for task in tasks {
        let status = if task.done { "[x]" } else { "[ ]" };
        println!("{} #{}: {}", status, task.id, task.description);
    }
}

fn main() {
    let mut tasks = load_tasks();
    
    println!("=== Rust Task Manager ===");
    add_task(&mut tasks, "Build Rust CLI tool".to_string());
    add_task(&mut tasks, "Write unit tests".to_string());
    add_task(&mut tasks, "Deploy to production".to_string());
    
    tasks[0].done = true;
    
    list_tasks(&tasks);
    save_tasks(&tasks);
    println!("Tasks saved to {}", DB_FILE);
}
