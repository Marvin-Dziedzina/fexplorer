pub mod directory;
pub mod error;
pub mod file;
pub mod link;
pub mod traits;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub use directory::{Child, Directory};
pub use error::Error;
pub use file::File;
pub use link::{Link, LinkType};
pub use traits::PathTrait;

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
    ) -> Result<(HashMap<PathBuf, Directory>, Vec<PathBuf>), Error> {
        let mut map = HashMap::new();
        let mut parent_children = Vec::new();

        let directories = match Self::get_directories(path) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };
        for entry in directories {
            parent_children.push(entry.clone());

            let (recursive_found_dirs, children) = match Self::get_directories_recursive(&entry) {
                Ok(recursive_found_dirs) => recursive_found_dirs,
                Err(_) => continue,
            };
            map.extend(recursive_found_dirs);

            let directory = match Directory::new(&entry, Some(children)) {
                Ok(directory) => directory,
                Err(_) => continue,
            };
            map.insert(entry, directory);
        }

        Ok((map, parent_children))
    }

    pub fn index_directories(&self) -> Result<HashMap<PathBuf, Directory>, Error> {
        let (map, _) = match Self::get_directories_recursive(&self.path) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        Ok(map)
    }

    fn get_files_recursive(path: &Path) -> Result<HashMap<PathBuf, File>, Error> {
        let mut map = HashMap::new();

        let files = match Self::get_files(path) {
            Ok(files) => files,
            Err(e) => return Err(e),
        };
        // add files to map
        for file_path in files {
            let file = match File::new(&file_path) {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            map.insert(file_path, file);
        }

        let directories = match Self::get_directories(path) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };
        // get files from child dirs
        for entry in directories {
            let recursive_found_files = match Self::get_files_recursive(&entry) {
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

        let links = match Self::get_links(path) {
            Ok(links) => links,
            Err(e) => return Err(e),
        };
        // add links to map
        for link in links {
            let link_path = match link {
                LinkType::Directory(link_path) => link_path,
                LinkType::File(link_path) => link_path,
            };

            let link = match Link::new(&link_path) {
                Ok(link) => link,
                Err(_) => continue,
            };

            map.insert(link_path, link);
        }

        let directories = match Self::get_directories(path) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };
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

    pub fn index_links(&self) -> Result<HashMap<PathBuf, Link>, Error> {
        match Self::get_links_recursive(&self.path) {
            Ok(links) => Ok(links),
            Err(e) => Err(e),
        }
    }
}

// implement get dir/file/link functions
impl Indexer {
    fn get_directories(path: &Path) -> Result<Vec<PathBuf>, Error> {
        let dirs = match fs::read_dir(path) {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(Error::IO(e)),
        };

        let mut entries = Vec::new();

        for entry in dirs {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let entry_path = entry.path();

            // Check if is link
            let is_link = match fs::read_link(&path) {
                Ok(_) => true,   // Is a link
                Err(_) => false, // Is not a link
            };

            if entry_path.is_dir() && !is_link {
                // is directory
                entries.push(entry_path);
            };
        }

        Ok(entries)
    }

    fn get_files(path: &Path) -> Result<Vec<PathBuf>, Error> {
        let dirs = match fs::read_dir(path) {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(Error::IO(e)),
        };

        let mut entries = Vec::new();

        for entry in dirs {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let entry_path = entry.path();

            // Check if is link
            let is_link = match fs::read_link(&path) {
                Ok(_) => true,   // Is a link
                Err(_) => false, // Is not a link
            };

            if entry_path.is_file() && !is_link {
                // is file
                entries.push(entry_path);
            };
        }

        Ok(entries)
    }

    fn get_links(path: &Path) -> Result<Vec<LinkType>, Error> {
        let dirs = match fs::read_dir(path) {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(Error::IO(e)),
        };

        let mut entries = Vec::new();

        for entry in dirs {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let entry_path = entry.path();

            // Check if is link
            let is_link = match fs::read_link(&path) {
                Ok(_) => true,   // Is a link
                Err(_) => false, // Is not a link
            };

            if is_link && entry_path.is_dir() {
                // is file
                entries.push(LinkType::Directory(entry_path));
            } else if is_link && entry_path.is_file() {
                entries.push(LinkType::File(entry_path));
            };
        }

        Ok(entries)
    }
}
