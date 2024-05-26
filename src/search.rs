use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{file_system, FexplorerError};

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

    fn get_directories_recursive(
        path: &Path,
    ) -> Result<HashMap<PathBuf, Directory>, FexplorerError> {
        let mut map = HashMap::new();
        let mut parent_children = Vec::new();

        let (directories, _, _) = file_system::get_entries_sorted(path)?;

        for directory in directories {
            parent_children.push(directory.clone());

            let recursive_found_dirs = match Self::get_directories_recursive(&directory) {
                Ok(recursive_found_dirs) => recursive_found_dirs,
                Err(_) => continue,
            };
            map.extend(recursive_found_dirs);

            let directory_obj = match Directory::new(&directory) {
                Ok(directory_obj) => directory_obj,
                Err(_) => continue,
            };

            map.insert(directory, directory_obj);
        }

        Ok(map)
    }

    pub fn index_directories(&self) -> Result<HashMap<PathBuf, Directory>, FexplorerError> {
        let map = match Self::get_directories_recursive(&self.path) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        Ok(map)
    }

    fn get_files_recursive(path: &Path) -> Result<HashMap<PathBuf, File>, FexplorerError> {
        let mut map = HashMap::new();

        let (directories, files, _) = file_system::get_entries_sorted(path)?;

        // add files to map
        for file in files {
            let file_obj = match File::new(&file) {
                Ok(file_obj) => file_obj,
                Err(_) => continue,
            };
            map.insert(file, file_obj);
        }

        // get files from child dirs
        for directory in directories {
            let recursive_found_files = match Self::get_files_recursive(&directory) {
                Ok(recursive_found_files) => recursive_found_files,
                Err(_) => continue,
            };

            map.extend(recursive_found_files);
        }

        Ok(map)
    }

    pub fn index_files(&self) -> Result<HashMap<PathBuf, File>, FexplorerError> {
        match Self::get_files_recursive(&self.path) {
            Ok(files) => Ok(files),
            Err(e) => Err(e),
        }
    }

    fn get_links_recursive(path: &Path) -> Result<HashMap<PathBuf, Link>, FexplorerError> {
        let mut map = HashMap::new();

        let (directories, _, links) = file_system::get_entries_sorted(path)?;

        // add links to map
        for link in links {
            let link_obj = match Link::new(&link) {
                Ok(link_obj) => link_obj,
                Err(_) => continue,
            };

            map.insert(link, link_obj);
        }

        // get links from child dirs
        for entry in directories {
            let recursive_found_links = match Self::get_links_recursive(&entry) {
                Ok(recursive_found_links) => recursive_found_links,
                Err(_) => continue,
            };

            map.extend(recursive_found_links);
        }

        Ok(map)
    }

    pub fn index_links(&self) -> Result<HashMap<PathBuf, Link>, FexplorerError> {
        match Self::get_links_recursive(&self.path) {
            Ok(links) => Ok(links),
            Err(e) => Err(e),
        }
    }
}
