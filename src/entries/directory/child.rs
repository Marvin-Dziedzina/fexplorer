use serde::{Deserialize, Serialize};

use super::{Directory, PathTrait};

use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Child {
    dir_name: Box<PathBuf>,
}

impl Child {
    pub fn new(dir_name: &Path) -> Self {
        Self {
            dir_name: Box::new(dir_name.to_owned()),
        }
    }

    pub fn from_str(dir_name: &str) -> Self {
        Self::new(Path::new(dir_name))
    }

    pub fn from_path(path: &Path) -> Self {
        Self::new(&PathBuf::from(Self::get_name_from_path(path)))
    }

    pub fn get_full_path(&self, directory: &Directory) -> PathBuf {
        let mut path = directory.get_path().to_path_buf();
        path.push(&self.dir_name.to_path_buf());

        path
    }

    fn get_name_from_path(path: &Path) -> String {
        match path.file_name() {
            Some(name_str) => name_str.to_string_lossy().to_string(),
            None => path.to_string_lossy().to_string(),
        }
    }
}

impl PathTrait for Child {
    fn get_path(&self) -> &Box<PathBuf> {
        &self.dir_name
    }

    fn get_metadata(&self) -> Result<std::fs::Metadata, std::io::Error> {
        self.dir_name.metadata()
    }

    fn get_name(&self) -> String {
        self.dir_name.to_string_lossy().to_string()
    }
}
