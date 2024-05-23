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
pub use link::Link;
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

    fn get_folders_recursive(
        path: &Path,
    ) -> Result<(HashMap<PathBuf, Directory>, Vec<PathBuf>), Error> {
        let dirs = match fs::read_dir(path) {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(Error::IO(e)),
        };

        let mut map = HashMap::new();
        let mut parent_children = Vec::new();

        for entry in dirs {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let path = entry.path();
            if path.is_dir() {
                match fs::read_link(&path) {
                    // Check if is link
                    Ok(_) => continue, // Is a link
                    Err(_) => (),      // Is not a link
                };
            } else {
                continue;
            }

            parent_children.push(path.to_path_buf());

            let (child_dirs, children) = match Self::get_folders_recursive(&path) {
                Ok(res) => res,
                Err(_) => continue,
            };

            let mut dir = match Directory::new(&path, None) {
                Ok(dir) => dir,
                Err(e) => panic!("{}", e),
            };

            dir.add_children(children);

            map.insert(path, dir);
            map.extend(child_dirs);
        }

        Ok((map, parent_children))
    }

    pub fn index_folders(&self) -> Result<HashMap<PathBuf, Directory>, Error> {
        let (map, _) = match Self::get_folders_recursive(&self.path) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        Ok(map)
    }

    fn get_files_recursive(path: &Path) -> Result<HashMap<PathBuf, File>, Error> {
        let dirs = match fs::read_dir(path) {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(Error::IO(e)),
        };

        let mut map = HashMap::new();

        for entry in dirs {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let path = entry.path();
            // Check if is link
            match fs::read_link(&path) {
                Ok(_) => continue, // Is a link
                Err(_) => (),      // Is not a link
            };

            if path.is_dir() {
                let files = match Self::get_files_recursive(&path) {
                    Ok(files) => files,
                    Err(_) => continue,
                };

                map.extend(files);

                continue;
            };

            let file = match File::new(&path) {
                Ok(file) => file,
                Err(_) => continue,
            };
            map.insert(path, file);
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
        let dirs = match fs::read_dir(path) {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(Error::IO(e)),
        };

        let mut map = HashMap::new();

        for entry in dirs {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let path = entry.path();

            // Check if is link
            let is_link = match fs::read_link(&path) {
                Ok(_) => true,   // Is a link
                Err(_) => false, // Is not a link
            };

            if path.is_dir() && !is_link {
                let links = match Self::get_links_recursive(&path) {
                    Ok(links) => links,
                    Err(_) => continue,
                };

                map.extend(links);

                continue;
            };

            if !is_link || !path.is_symlink() {
                continue;
            }

            let link = match Link::new(&path) {
                Ok(link) => link,
                Err(_) => continue,
            };
            map.insert(path, link);
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
