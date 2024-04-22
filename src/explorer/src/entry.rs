use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

use crate::enums;

use enums::EntryType;

pub struct Entry {
    entry_type: EntryType,
    name: Box<OsString>,
    path: Box<PathBuf>,
    has_children: bool,
}
impl Entry {
    pub fn new(path: &PathBuf) -> Result<Self, Error> {
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

        // get EntryType
        let entry_type: EntryType;
        if path.is_dir() {
            entry_type = EntryType::Directory;
        } else if path.is_file() {
            entry_type = EntryType::File;
        } else if path.is_symlink() {
            entry_type = EntryType::Link;
        } else {
            entry_type = EntryType::Unknown;
        };

        // get has_children
        let has_children = match fs::read_dir(path) {
            Ok(children) => children.count() > 0,
            Err(e) => return Err(Error::IO(e)),
        };

        Ok(Self {
            entry_type,
            name: Box::new(name.to_owned()),
            path: Box::new(path.to_owned()),
            has_children: has_children,
        })
    }

    pub fn get_type(&self) -> &EntryType {
        &self.entry_type
    }

    pub fn get_name(&self) -> Box<OsString> {
        self.name.clone()
    }

    pub fn get_path(&self) -> Box<PathBuf> {
        self.path.clone()
    }

    pub fn get_rel_path(&self) -> Result<Box<PathBuf>, Error> {
        let name = match self.name.clone().into_string() {
            Ok(name) => name,
            Err(_) => {
                return Err(Error::ConversionFailure(
                    "Could not convert OsString to String!".to_owned(),
                ))
            }
        };

        Ok(Box::new(Path::new("").join(name)))
    }

    pub fn has_children(&self) -> bool {
        self.has_children
    }
}

#[derive(Debug)]
pub enum Error {
    PathDoesNotExist(String),
    FaultyName(String),
    ConversionFailure(String),
    IO(std::io::Error),
}
