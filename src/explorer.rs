use std::env;
use std::path::{Path, PathBuf};

use opener;

pub mod entries;

use crate::{file_system, FexplorerError};

type Directory = PathBuf;
type File = PathBuf;
type Link = PathBuf;

pub struct Explorer {
    path: PathBuf,
    directories: Vec<PathBuf>,
    files: Vec<PathBuf>,
    links: Vec<PathBuf>,
}
impl Explorer {
    pub fn new(path: &Path) -> Result<Self, FexplorerError> {
        let (directories, files, links) = file_system::get_entries_sorted(path)?;

        Ok(Self {
            path: path.to_owned(),
            directories,
            files,
            links,
        })
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_entries(&self) -> (&Vec<Directory>, &Vec<File>, &Vec<Link>) {
        (&self.directories, &self.files, &self.links)
    }

    fn update_entries(&mut self) -> Result<(), FexplorerError> {
        (self.directories, self.files, self.links) = file_system::get_entries_sorted(&self.path)?;

        Ok(())
    }

    pub fn set_path(&mut self, path: &Path) -> Result<(), FexplorerError> {
        if file_system::is_directory(path) {
            self.path = path.to_owned();
        } else if file_system::is_file(path) {
            match Self::open_file_with_default_app(path) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        } else if file_system::is_link_to_directory(path) {
            self.path = match file_system::get_link_target(path) {
                Ok(target_path) => target_path,
                Err(e) => return Err(e),
            };
        } else if file_system::is_link_to_file(path) {
            match Self::open_file_with_default_app(path) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }

        match self.update_entries() {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e),
        }
    }

    fn open_file_with_default_app(path: &Path) -> Result<(), FexplorerError> {
        match opener::open(path) {
            Ok(_) => (),
            Err(e) => return Err(FexplorerError::OpenError(e)),
        };

        Ok(())
    }

    pub fn add_path(&mut self, rel_path: &Path) -> Result<(), FexplorerError> {
        match self.set_path(&self.path.join(rel_path)) {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e),
        }
    }

    pub fn set_to_parent(&mut self) -> Result<(), FexplorerError> {
        let parent = match self.path.parent() {
            Some(parent) => parent,
            None => &self.path,
        };

        let path = match parent.canonicalize() {
            Ok(parent) => parent,
            Err(_) => parent.to_path_buf(),
        };

        self.set_path(&path)?;

        Ok(())
    }
}

impl Default for Explorer {
    fn default() -> Self {
        let path = env::current_dir().unwrap();
        Self::new(&path).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_entries(entries: (&Vec<Directory>, &Vec<File>, &Vec<Link>)) {
        for path in entries.0 {
            println!("{:?}", path);
        }

        for path in entries.1 {
            println!("{:?}", path);
        }

        for path in entries.2 {
            println!("{:?}", path);
        }
    }

    #[test]
    fn new() {
        let explorer = Explorer::new(Path::new(&env::current_dir().unwrap())).unwrap();
        print_entries(explorer.get_entries());
    }
}
