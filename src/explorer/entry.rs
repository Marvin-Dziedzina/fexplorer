use std::{
    fs,
    path::{Path, PathBuf},
    ptr::eq,
};

use super::enums;
use super::traits::BasicEntry;
use crate::file_system::error::Error;
use enums::EntryType;

#[derive(Debug)]
pub struct Entry {
    entry_type: EntryType,
    name: String,
    path: PathBuf,
    has_children: bool,
}
impl BasicEntry for Entry {
    fn new(path: &Path) -> Result<Self, Error> {
        match path.try_exists() {
            Ok(_) => (),
            Err(_) => {
                return Err(Error::PathDoesNotExist(format!(
                    "'{}' does not exist!",
                    path.to_string_lossy()
                )))
            }
        };

        // get name
        let name = match path.file_name() {
            Some(name) => name,
            None => return Err(Error::FaultyName("The folder name is faulty!".to_owned())),
        };

        let entry_type = Self::get_entry_type_from_path(&path);

        // get has_children
        let mut has_children = false;
        if eq(&entry_type, &EntryType::Directory) {
            has_children = match fs::read_dir(path) {
                Ok(children) => children.count() > 0,
                Err(e) => return Err(Error::IO(e)),
            };
        }

        Ok(Self {
            entry_type,
            name: name.to_string_lossy().to_string(),
            path: path.to_owned(),
            has_children,
        })
    }

    fn get_type(&self) -> &EntryType {
        &self.entry_type
    }

    fn get_name(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    fn get_rel_path(&self) -> Result<PathBuf, Error> {
        let name = self.name.clone();

        Ok(Path::new("").join(name))
    }

    fn has_children(&self) -> bool {
        self.has_children
    }
}
