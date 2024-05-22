pub mod directory;
pub mod file;
pub mod link;
pub mod traits;

use std::{
    collections::HashMap,
    fs::{self, ReadDir},
    io,
    path::{Path, PathBuf},
};

pub use directory::{Child, Directory};
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

    /// Test function
    pub fn get_paths(&self) -> HashMap<PathBuf, PathBuf> {
        fn read_dir(path: &PathBuf) -> Result<fs::ReadDir, io::Error> {
            match fs::read_dir(path) {
                Ok(read_dir) => Ok(read_dir),
                Err(e) => Err(e),
            }
        }

        fn index(dirs: ReadDir, mut result: HashMap<PathBuf, PathBuf>) -> HashMap<PathBuf, PathBuf> {
            for entry in dirs {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };
                
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                match fs::read_link(&path) { // Check if is link
                    Ok(_) => continue, // Is a link
                    Err(_) => (), // Is not a link
                };

                let dirs = match read_dir(&path) {
                    Ok(dirs) => dirs,
                    Err(_) => continue,
                };
                
                result.insert(path.clone(), path);
                result = index(dirs, result);
            };

            result
        }

        let dirs = match read_dir(&self.path) {
            Ok(dirs) => dirs,
            Err(e) => panic!("Remove from source code! Error: {}", e),
        };

        index(dirs, HashMap::new())
    }

    pub fn index_folders(&self) -> Result<HashMap<PathBuf, Directory>, io::Error> {
        fn read_dir(path: &PathBuf) -> Result<fs::ReadDir, io::Error> {
            match fs::read_dir(path) {
                Ok(read_dir) => Ok(read_dir),
                Err(e) => Err(e),
            }
        }

        fn index(dirs: ReadDir) -> (HashMap<PathBuf, Directory>, Vec<PathBuf>) {
            let mut map = HashMap::new();
            let mut parent_children = Vec::new();

            for entry in dirs {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };

                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                match fs::read_link(&path) { // Check if is link
                    Ok(_) => continue, // Is a link
                    Err(_) => (), // Is not a link
                };
                
                parent_children.push(path.to_path_buf());

                let child_dirs = match read_dir(&path) {
                    Ok(child_dirs) => child_dirs,
                    Err(_) => continue,
                };

                let (child_dirs, children) = index(child_dirs);

                let mut dir = Directory::new(&path, None);
                if !children.is_empty() {
                    dir.add_children(children);
                };
                map.insert(path, dir);

                if !child_dirs.is_empty() {
                    map.extend(child_dirs);
                };
            };

            (map, parent_children)
        }

        let dirs = match read_dir(&self.path) {
            Ok(dirs) => dirs,
            Err(e) => return Err(e),
        };

        let (map, _) = index(dirs);

        Ok(map)
    }

    pub fn index_file(start: &Path) -> HashMap<PathBuf, File> {
        todo!()
    }

    pub fn index_link(start: &Path) -> HashMap<PathBuf, Link> {
        todo!()
    }
}
