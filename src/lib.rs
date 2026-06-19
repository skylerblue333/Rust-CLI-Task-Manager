use std::collections::HashMap;
use std::sync::Mutex;

pub struct TaskManager {
    tasks: Mutex<HashMap<String, String>>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Mutex::new(HashMap::new()) }
    }
    pub fn add(&self, id: String, cmd: String) {
        self.tasks.lock().unwrap().insert(id, cmd);
    }
    pub fn get(&self, id: &str) -> Option<String> {
        self.tasks.lock().unwrap().get(id).cloned()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_manager() {
        let tm = TaskManager::new();
        tm.add("t1".to_string(), "echo hello".to_string());
        assert_eq!(tm.get("t1"), Some("echo hello".to_string()));
        assert_eq!(tm.get("t2"), None);
    }

}
