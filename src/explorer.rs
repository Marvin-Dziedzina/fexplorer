use std::env;
use std::path::{Path, PathBuf};

pub mod entries;
pub mod error;

use super::entries::{Directory, File, Link};
pub use entries::Entries;
pub use error::Error;

pub struct Explorer {
    path: PathBuf,
    entries: Entries,
}
impl Explorer {
    pub fn new(path: &Path) -> Result<Self, Error> {
        Ok(Self {
            path: path.to_owned(),
            entries: Entries::new(path)?,
        })
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_entries(&self) -> &Entries {
        &self.entries
    }

    pub fn get_entries_from_path(
        path: &Path,
    ) -> Result<(Vec<Directory>, Vec<File>, Vec<Link>), Error> {
        let entries = match Entries::new(path) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };
        let (directories, files, links) = entries.get_entries();

        Ok((directories, files, links))
    }

    fn update_entries(&mut self) -> Result<(), Error> {
        self.entries = match Entries::new(&self.path) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    pub fn set_path(&mut self, path: &Path) -> Result<(), Error> {
        self.path = path.to_owned();

        match self.update_entries() {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e),
        }
    }

    pub fn add_path(&mut self, rel_path: &Path) -> Result<(), Error> {
        match self.set_path(&self.path.join(rel_path)) {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e),
        }
    }

    pub fn set_to_parent(&mut self) -> Result<(), Error> {
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

    fn get_entries_boxed(
        path: &Path,
    ) -> Result<(Box<Vec<Directory>>, Box<Vec<File>>, Box<Vec<Link>>), Error> {
        match Explorer::get_entries_from_path(path) {
            Ok((directories, files, links)) => {
                Ok((Box::new(directories), Box::new(files), Box::new(links)))
            }
            Err(e) => return Err(e),
        }
    }
}

impl Default for Explorer {
    fn default() -> Self {
        let path = env::current_dir().unwrap();
        Self::new(&path).unwrap()
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;

    fn print_entries(entries: Entries) {
        let paths = entries.get_paths();

        for path in paths {
            println!("{:?}", path);
        }
    }

    #[test]
    fn new() {
        let explorer = Explorer::new(Path::new(&env::current_dir().unwrap())).unwrap();
        let entries = explorer.get_entries();
        print_entries(entries);
    }

    #[test]
    fn test_parent() {
        let mut explorer = Explorer::new(Path::new(&env::current_dir().unwrap())).unwrap();

        explorer.set_to_parent().unwrap();
        print_entries(&explorer);
    }
} */
