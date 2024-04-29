use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{explorer::enums::EntryType, file_system::traits::BasicEntry};

use self::search_entry::SearchEntry;

pub mod error;
pub mod search_entry;

pub struct Search {}
impl Search {
    pub fn index_path(path: &Path) -> HashMap<PathBuf, SearchEntry> {
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(e) => {
                println!("Error on {}: {}", path.to_string_lossy(), e);
                return HashMap::new();
            }
        };

        let mut search_entries = HashMap::new();

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let path = entry.path();
            let mut search_entry = match SearchEntry::new(&path) {
                Ok(search_entry) => search_entry,
                Err(_) => continue,
            };

            if matches!(
                SearchEntry::get_entry_type_from_path(&path),
                EntryType::Directory,
            ) {
                let new_entries = Self::index_path(&path);
                for (i, _) in &new_entries {
                    let parent_path = match i.parent() {
                        Some(parent_path) => parent_path,
                        None => i,
                    };

                    if &parent_path == &path {
                        search_entry.add_child(i.clone());
                    };
                }

                search_entries.extend(new_entries)
            };

            search_entries.insert(path, search_entry);
        }

        search_entries
    }
}
