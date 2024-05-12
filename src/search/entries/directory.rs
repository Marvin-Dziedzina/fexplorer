use super::traits::PathTrait;

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

mod child;
pub use child::Child;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Directory {
    path: Box<PathBuf>,
    children: HashMap<String, Child>,
}
impl Directory {
    pub fn new(path: &Path, children: Option<HashMap<String, Child>>) -> Self {
        let children = match children {
            Some(children) => children,
            None => HashMap::new(),
        };

        Self {
            path: Box::new(path.to_path_buf()),
            children,
        }
    }

    pub fn get_children(&self) -> &HashMap<String, Child> {
        &self.children
    }

    pub fn get_child_by_string(&self, name: &str) -> Option<&Child> {
        self.children.get(name)
    }

    pub fn add_child(&mut self, directory: &Directory) {
        let path = directory.get_path().to_path_buf();
        let name = match path.file_name() {
            Some(name_str) => name_str.to_string_lossy().to_string(),
            None => path.to_string_lossy().to_string(),
        };

        let child: Child = Child::new(Path::new(&name));

        self.children.insert(name, child);
    }
}

impl PathTrait for Directory {
    fn get_path(&self) -> &Box<PathBuf> {
        &self.path
    }

    fn get_metadata(&self) -> Result<fs::Metadata, io::Error> {
        self.path.metadata()
    }

    fn get_name(&self) -> String {
        match self.path.file_name() {
            Some(name_str) => name_str.to_string_lossy().to_string(),
            None => self.path.to_string_lossy().to_string(),
        }
    }
}
