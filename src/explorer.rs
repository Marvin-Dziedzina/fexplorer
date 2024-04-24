use std::env;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub mod entry;
pub mod enums;
pub mod error;

use entry::Entry;

use self::enums::{EntryType, ErrorAddPath};

pub struct Explorer {
    path: PathBuf,
    entries: Box<Vec<Entry>>,
}
impl Explorer {
    pub fn new(path: &Path) -> Result<Self, io::Error> {
        let entries = match Explorer::get_entries_from_path(&path) {
            Ok(entries) => Box::new(entries),
            Err(e) => return Err(e),
        };

        Ok(Self {
            path: path.to_owned(),
            entries,
        })
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn get_entries_from_path(path: &Path) -> Result<Vec<Entry>, io::Error> {
        let mut entries: Vec<Entry> = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let entry = match Entry::new(&entry.path()) {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            entries.push(entry);
        }

        Ok(entries)
    }

    fn update_entries(&mut self) -> Result<(), io::Error> {
        self.entries = match Explorer::get_entries_from_path(&self.path) {
            Ok(entries) => Box::new(entries),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    pub fn set_path(&mut self, path: &Path) -> Result<(), io::Error> {
        self.path = path.to_owned();

        match self.update_entries() {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e),
        }
    }

    pub fn add_path(&mut self, rel_path: &Path) -> Result<(), ErrorAddPath> {
        let path = self.path.join(rel_path);

        match Entry::get_entry_type_from_path(&path) {
            EntryType::Directory => match self.set_path(&path) {
                Ok(_) => Ok(()),
                Err(e) => Err(ErrorAddPath::IO(e)),
            },
            EntryType::File => {
                let mut errors = Vec::new();

                for mut command in open::commands(path) {
                    match command.status() {
                        Ok(_) => {
                            return Ok(());
                        }
                        Err(e) => {
                            errors.push(e);
                            continue;
                        }
                    }
                }

                Err(ErrorAddPath::IoVec(errors))
            }
            EntryType::Link => todo!(),
            EntryType::Unknown => todo!(),
        }
    }

    pub fn set_to_parent(&mut self) -> Result<(), io::Error> {
        let path = match self.path.parent() {
            Some(parent) => match parent.canonicalize() {
                Ok(parent) => parent,
                Err(e) => return Err(e),
            },
            None => self.path.to_owned(),
        };

        self.set_path(&path)?;

        Ok(())
    }
}

impl Default for Explorer {
    fn default() -> Self {
        let path = env::current_dir().unwrap();
        let entries = Explorer::get_entries_from_path(&path).unwrap();

        Self {
            path,
            entries: Box::new(entries),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_entries(explorer: &Explorer) {
        println!("[Location] {}", explorer.get_path().to_string_lossy());

        for entry in explorer.get_entries() {
            println!(
                "[{}] {}, {}, has_children: {}",
                entry.get_type(),
                entry.get_name().into_string().unwrap(),
                entry.get_path().to_string_lossy(),
                entry.has_children(),
            );
        }
    }

    #[test]
    fn new() {
        let explorer = Explorer::new(Path::new(&env::current_dir().unwrap())).unwrap();
        print_entries(&explorer);
    }

    #[test]
    fn test_parent() {
        let mut explorer = Explorer::new(Path::new(&env::current_dir().unwrap())).unwrap();

        explorer.set_to_parent().unwrap();
        print_entries(&explorer);
    }

    #[test]
    fn add_path() {
        let mut explorer = Explorer::new(Path::new(&env::current_dir().unwrap())).unwrap();
        let entry = explorer.get_entries().get(0).unwrap();

        let rel_path = entry.get_rel_path().unwrap();
        explorer.add_path(&rel_path).unwrap();
        print_entries(&explorer);
    }
}
