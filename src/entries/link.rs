use serde::{Deserialize, Serialize};

use crate::{file_system, FexplorerError};

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
    pub fn new(path: &Path) -> Result<Self, FexplorerError> {
        if !file_system::is_link(path) {
            return Err(FexplorerError::FileSystem(
                file_system::FileSystemError::NotALink(path.to_path_buf()),
            ));
        }

        let target_path = match file_system::get_link_target(path) {
            Ok(path) => path,
            Err(e) => return Err(e),
        };

        Ok(Self {
            path: Box::new(path.to_path_buf()),
            target: Box::new(target_path),
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
