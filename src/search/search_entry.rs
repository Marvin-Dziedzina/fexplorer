use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    thread::{self, JoinHandle},
};

use serde::{Deserialize, Serialize};

use crate::explorer::enums::EntryType;
use crate::file_system::error::Error;
use crate::file_system::traits::BasicEntry;

use super::SearchEntryThreadData;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchEntry {
    entry_type: EntryType,
    name: String,
    path: Box<PathBuf>,
    has_children: bool,
    children: Vec<SearchEntry>,
}
impl BasicEntry for SearchEntry {
    fn new(path: PathBuf) -> Result<Self, Error>
    where
        Self: Sized,
    {
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

        // get entry type
        let entry_type = Self::get_entry_type_from_path(&path);

        // get entries
        let children_join_handle = Self::index_children(path.clone());
        let children = match children_join_handle.join() {
            Ok(children) => children,
            Err(_) => return Err(Error::Generic(String::from("Could not get children!"))),
        };

        // get has_children
        let has_children = children.len() > 0;

        Ok(Self {
            entry_type,
            name: name.to_string_lossy().to_string(),
            path: Box::new(path.to_owned()),
            has_children,
            children,
        })
    }

    fn get_type(&self) -> &EntryType {
        &self.entry_type
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_path(&self) -> &Box<PathBuf> {
        &self.path
    }

    fn get_rel_path(&self) -> Result<Box<PathBuf>, Error> {
        let name = self.name.clone();

        Ok(Box::new(Path::new("").join(name)))
    }

    fn has_children(&self) -> bool {
        self.has_children
    }
}

impl SearchEntry {
    fn from_children(path: &Path, children: Vec<SearchEntry>) -> Result<Self, Error> {
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
        let has_children = children.len() > 0;

        Ok(Self {
            entry_type,
            name: name.to_string_lossy().to_string(),
            path: Box::new(path.to_owned()),
            has_children,
            children,
        })
    }

    fn index_children(path: PathBuf) -> JoinHandle<Vec<SearchEntry>> {
        thread::spawn(move || {
            let mut children: Vec<SearchEntry> = Vec::new();
            let mut threads: Vec<SearchEntryThreadData> = Vec::new();

            let read_dir = match fs::read_dir(&path) {
                Ok(read_dir) => read_dir,
                Err(e) => panic!("Panic: {}", e),
            };

            for entry in read_dir {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };

                let entry_path = entry.path();

                if entry_path.is_dir() {
                    threads.push(SearchEntryThreadData::new(
                        entry_path.clone(),
                        Self::index_children(entry_path),
                    ));
                    continue;
                };

                let search_entry = match Self::new(entry_path) {
                    Ok(search_entry) => search_entry,
                    Err(_) => continue,
                };

                children.push(search_entry);
            }

            for search_entry_thread_data in threads {
                let curr_path = search_entry_thread_data.get_path();

                let search_entries =
                    match search_entry_thread_data.get_children_join_handle().join() {
                        Ok(search_entries) => search_entries,
                        Err(_) => continue,
                    };

                match Self::from_children(&curr_path, search_entries) {
                    Ok(search_entry) => children.push(search_entry),
                    Err(_) => continue,
                };
            }

            return children;
        })
    }
}
