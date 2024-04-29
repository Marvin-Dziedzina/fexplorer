use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::explorer::enums::EntryType;
use crate::file_system::error::Error;
use crate::file_system::traits::BasicEntry;

use super::error::SearchError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchEntry {
    entry_type: EntryType,
    path: PathBuf,
    children: Vec<PathBuf>,
}
impl BasicEntry for SearchEntry {
    fn new(path: &Path) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let path = path.to_path_buf();

        match path.try_exists() {
            Ok(_) => (),
            Err(_) => {
                return Err(Error::PathDoesNotExist(format!(
                    "'{}' does not exist!",
                    path.to_string_lossy()
                )))
            }
        };

        // get entry type
        let entry_type = Self::get_entry_type_from_path(&path);

        Ok(Self {
            entry_type,
            path,
            children: Vec::new(),
        })
    }

    fn get_type(&self) -> &EntryType {
        &self.entry_type
    }

    fn get_name(&self) -> Option<String> {
        match self.path.file_name() {
            Some(name) => Some(name.to_string_lossy().to_string()),
            None => None,
        }
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    fn get_rel_path(&self) -> Result<PathBuf, Error> {
        let name = match self.get_name() {
            Some(name) => name.clone(),
            None => return Err(Error::Generic(String::from("Could not get name!"))),
        };
        let rel_path = match PathBuf::from_str(&name) {
            Ok(rel_path) => rel_path,
            Err(e) => return Err(Error::Infallible(e)),
        };

        Ok(rel_path)
    }

    fn has_children(&self) -> bool {
        self.children.len() > 0
    }
}

impl SearchEntry {
    /// Add a child. The child argument needs to be the relpath to the child.
    pub fn add_child(&mut self, child: PathBuf) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, i: usize) {
        self.children.swap_remove(i);
    }

    pub fn remove_child_by_value(&mut self, path: PathBuf) -> Result<(), SearchError> {
        let mut index = None;
        for (i, v) in self.children.iter().enumerate() {
            if path.to_string_lossy() != v.to_string_lossy() {
                continue;
            }

            index = Some(i);
            break;
        }

        match index {
            Some(index) => self.children.remove(index),
            None => {
                return Err(SearchError::ChildNotFound(String::from(
                    "Could not find child!",
                )))
            }
        };

        Ok(())
    }
}
