use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub tasks: Vec<String>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    pub fn load(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let list = serde_json::from_str(&content)?;
        Ok(list)
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())
    }

    pub fn add(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index < self.tasks.len() {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }
}