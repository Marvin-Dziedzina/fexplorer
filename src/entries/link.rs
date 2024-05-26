use serde::{Deserialize, Serialize};

use crate::explorer::Error;

use super::traits::PathTrait;

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
            Ok(link_data) => link_data, // Is a link
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

#[derive(Debug, Serialize, Deserialize)]
pub enum LinkType {
    File(PathBuf),
    Directory(PathBuf),
}
impl LinkType {
    pub fn get_path(&self) -> PathBuf {
        match self {
            LinkType::Directory(path) => path.to_path_buf(),
            LinkType::File(path) => path.to_path_buf(),
        }
    }
}
