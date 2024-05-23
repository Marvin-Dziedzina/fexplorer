use serde::{Deserialize, Serialize};

use super::{traits::PathTrait, Error};

use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    path: Box<PathBuf>,
    target: Box<PathBuf>,
}
impl Link {
    pub fn new(path: &Path) -> Result<Self, Error> {
        // Check if is link
        let link_data = match fs::read_link(&path) {
            Ok(link_data) => link_data,                                                                // Is a link
            Err(_) => return Err(Error::InvalidEntryType(String::from("Not a link!"))), // Is not a link
        };

        Ok(Self {
            path: Box::new(path.to_path_buf()),
            target: Box::new(link_data),
        })
    }
}

impl PathTrait for Link {
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
