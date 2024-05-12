pub mod directory;
pub mod file;
pub mod link;
pub mod traits;

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

pub use directory::{Child, Directory};
pub use file::File;
pub use link::Link;
pub use traits::PathTrait;

pub fn index_directories(start: &Path) -> Result<HashMap<PathBuf, Directory>, io::Error> {
    let dirs = match fs::read_dir(start) {
        Ok(dirs) => dirs,
        Err(e) => return Err(e),
    };

    let mut map = HashMap::new();
    for entry in dirs {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();
        if !path.is_dir() {
            continue;
        };

        let children = match index_directories(&path) {
            Ok(children) => children,
            Err(_) => HashMap::new(),
        };

        // add all child names to dir
        let mut dir = Directory::new(&path, None);
        let mut test_path = path.clone();
        for (_, child) in &children {
            test_path.push(child.get_name());

            if test_path.is_dir() {
                dir.add_child(child)
            };

            test_path = match test_path.parent() {
                Some(path) => path.to_path_buf(),
                None => test_path,
            };
        }

        map.insert(path, dir);
        map.extend(children);
    }

    Ok(map)
}

pub fn index_file(start: &Path) -> HashMap<PathBuf, File> {
    todo!()
}

pub fn index_link(start: &Path) -> HashMap<PathBuf, Link> {
    todo!()
}
