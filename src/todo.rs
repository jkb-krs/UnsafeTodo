use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use dirs::data_local_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub tasks: Vec<String>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn get_data_path() -> PathBuf {
        let mut path = data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("unsafe_todo");
        fs::create_dir_all(&path).ok();
        path.push("todos.cbor");
        path
    }

    pub fn load() -> io::Result<Self> {
        let path = Self::get_data_path();
        if !path.exists() {
            return Ok(TodoList::new());
        }
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let list = serde_cbor::from_slice(&buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(list)
    }

    pub fn save(&self) -> io::Result<()> {
        let path = Self::get_data_path();
        let mut file = File::create(path)?;
        let data = serde_cbor::to_vec(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        file.write_all(&data)
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