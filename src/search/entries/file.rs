use serde::{Deserialize, Serialize};

use super::{traits::PathTrait, Error};

use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    path: Box<PathBuf>,
}
impl File {
    pub fn new(path: &Path) -> Result<Self, Error> {
        if !path.is_file() {
            return Err(Error::InvalidEntryType(String::from("Not a file!")));
        };

        Ok(Self {
            path: Box::new(path.to_path_buf()),
        })
    }
}

impl PathTrait for File {
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
