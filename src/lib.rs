use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub const MAX_TASKS: usize = 10_000;
pub const MAX_TITLE_LEN: usize = 200;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TaskStore {
    tasks: Vec<Task>,
}

impl TaskStore {
    pub fn load(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let mut file = File::open(path).map_err(|e| format!("open task file: {e}"))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|e| format!("read task file: {e}"))?;
        let store: Self = serde_json::from_str(&buf).map_err(|e| format!("parse task file: {e}"))?;
        if store.tasks.len() > MAX_TASKS {
            return Err("task file exceeds maximum supported task count".to_string());
        }
        Ok(store)
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("create task directory: {e}"))?;
        }
        let tmp = temp_path(path);
        let payload = serde_json::to_vec_pretty(self).map_err(|e| format!("serialize tasks: {e}"))?;
        let mut file = File::create(&tmp).map_err(|e| format!("create temp task file: {e}"))?;
        file.write_all(&payload)
            .map_err(|e| format!("write temp task file: {e}"))?;
        file.sync_all()
            .map_err(|e| format!("sync temp task file: {e}"))?;
        fs::rename(&tmp, path).map_err(|e| format!("replace task file: {e}"))?;
        Ok(())
    }

    pub fn add(&mut self, title: &str) -> Result<Task, String> {
        let title = title.trim();
        if title.is_empty() || title.chars().count() > MAX_TITLE_LEN {
            return Err(format!("title must contain 1-{MAX_TITLE_LEN} characters"));
        }
        if self.tasks.len() >= MAX_TASKS {
            return Err("task capacity reached".to_string());
        }
        let next_id = self
            .tasks
            .iter()
            .map(|task| task.id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or_else(|| "task id space exhausted".to_string())?;
        let task = Task {
            id: next_id,
            title: title.to_string(),
            completed: false,
        };
        self.tasks.push(task.clone());
        Ok(task)
    }

    pub fn complete(&mut self, id: u64) -> Result<Task, String> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or_else(|| "task not found".to_string())?;
        task.completed = true;
        Ok(task.clone())
    }

    pub fn remove(&mut self, id: u64) -> Result<Task, String> {
        let index = self
            .tasks
            .iter()
            .position(|task| task.id == id)
            .ok_or_else(|| "task not found".to_string())?;
        Ok(self.tasks.remove(index))
    }

    pub fn list(&self) -> &[Task] {
        &self.tasks
    }
}

fn temp_path(path: &Path) -> PathBuf {
    let mut tmp = path.as_os_str().to_os_string();
    tmp.push(".tmp");
    PathBuf::from(tmp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file(name: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("sky-cli-tasks-{name}-{stamp}.json"))
    }

    #[test]
    fn add_complete_remove_round_trip() {
        let mut store = TaskStore::default();
        let first = store.add("ship deterministic CLI").unwrap();
        assert_eq!(first.id, 1);
        assert!(!first.completed);
        let completed = store.complete(first.id).unwrap();
        assert!(completed.completed);
        let removed = store.remove(first.id).unwrap();
        assert_eq!(removed.title, "ship deterministic CLI");
        assert!(store.list().is_empty());
    }

    #[test]
    fn validates_title_bounds() {
        let mut store = TaskStore::default();
        assert!(store.add("   ").is_err());
        assert!(store.add(&"x".repeat(MAX_TITLE_LEN + 1)).is_err());
    }

    #[test]
    fn persists_atomically_and_reloads() {
        let path = temp_file("roundtrip");
        let mut store = TaskStore::default();
        store.add("persist me").unwrap();
        store.save(&path).unwrap();
        let loaded = TaskStore::load(&path).unwrap();
        assert_eq!(loaded.list(), store.list());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn missing_file_is_empty_store() {
        let path = temp_file("missing");
        let store = TaskStore::load(&path).unwrap();
        assert!(store.list().is_empty());
    }
}
