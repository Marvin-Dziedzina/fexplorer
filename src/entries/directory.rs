use crate::{
    file_system::{self, FileSystemError},
    FexplorerError,
};

use super::traits::PathTrait;

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

mod child;
pub use child::Child;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    path: Box<PathBuf>,
    children: HashMap<String, Child>,
}
impl Directory {
    pub fn new(path: &Path) -> Result<Self, FexplorerError> {
        if !path.is_dir() || file_system::is_link(path) {
            return Err(FexplorerError::FileSystem(FileSystemError::NotADirectory(
                path.to_path_buf(),
            )));
        };

        let mut children_map = HashMap::new();

        for entry in file_system::get_entries(path)? {
            children_map.insert(file_system::get_path_name(&entry), Child::from_path(&entry));
        }

        Ok(Self {
            path: Box::new(path.to_path_buf()),
            children: children_map,
        })
    }

    pub fn get_children(&self) -> &HashMap<String, Child> {
        &self.children
    }

    pub fn get_child_by_string(&self, name: &str) -> Option<&Child> {
        self.children.get(name)
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
