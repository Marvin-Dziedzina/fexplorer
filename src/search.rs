use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{
    entries::PathTrait,
    explorer::{entries::Entries, Error},
};

use super::entries::{directory::Directory, file::File, link::Link};

#[derive(Debug)]
pub struct Indexer {
    path: PathBuf,
}
impl Indexer {
    pub fn new(start: &Path) -> Self {
        Self {
            path: start.to_path_buf(),
        }
    }

    fn get_directories_recursive(path: &Path) -> Result<HashMap<PathBuf, Directory>, Error> {
        let mut map = HashMap::new();
        let mut parent_children = Vec::new();

        let entries = match Entries::new(path) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };

        let (directories, files, links) = entries.get_entries();
        for directory in directories {
            parent_children.push(directory.get_path().to_path_buf());

            let recursive_found_dirs = match Self::get_directories_recursive(&directory.get_path())
            {
                Ok(recursive_found_dirs) => recursive_found_dirs,
                Err(_) => continue,
            };
            map.extend(recursive_found_dirs);

            map.insert(directory.get_path().to_path_buf(), directory);
        }

        Ok(map)
    }

    pub fn index_directories(&self) -> Result<HashMap<PathBuf, Directory>, Error> {
        let map = match Self::get_directories_recursive(&self.path) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        Ok(map)
    }

    fn get_files_recursive(path: &Path) -> Result<HashMap<PathBuf, File>, Error> {
        let mut map = HashMap::new();

        let entries = match Entries::new(path) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };
        let (directories, files, links) = entries.get_entries();

        // add files to map
        for file in files {
            map.insert(file.get_path().to_path_buf(), file);
        }

        // get files from child dirs
        for entry in directories {
            let recursive_found_files = match Self::get_files_recursive(entry.get_path()) {
                Ok(recursive_found_files) => recursive_found_files,
                Err(_) => continue,
            };

            map.extend(recursive_found_files);
        }

        Ok(map)
    }

    pub fn index_files(&self) -> Result<HashMap<PathBuf, File>, Error> {
        match Self::get_files_recursive(&self.path) {
            Ok(files) => Ok(files),
            Err(e) => Err(e),
        }
    }

    fn get_links_recursive(path: &Path) -> Result<HashMap<PathBuf, Link>, Error> {
        let mut map = HashMap::new();

        let entries = match Entries::new(path) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };
        let (directories, files, links) = entries.get_entries();

        // add links to map
        for link in links {
            map.insert(link.get_path().to_path_buf(), link);
        }

        // get links from child dirs
        for entry in directories {
            let recursive_found_links = match Self::get_links_recursive(entry.get_path()) {
                Ok(recursive_found_links) => recursive_found_links,
                Err(_) => continue,
            };

            map.extend(recursive_found_links);
        }

        Ok(map)
    }

    pub fn index_links(&self) -> Result<HashMap<PathBuf, Link>, Error> {
        match Self::get_links_recursive(&self.path) {
            Ok(links) => Ok(links),
            Err(e) => Err(e),
        }
    }
}
