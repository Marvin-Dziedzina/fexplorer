use std::{
    collections::HashMap,
    fs,
    mem::take,
    path::{Path, PathBuf},
};

use egui::TextBuffer;

use crate::{explorer::enums::EntryType, file_system::traits::BasicEntry};

use self::search_entry::SearchEntry;

pub mod error;
pub mod search_entry;

pub struct Search {}
impl Search {
    pub fn index_path(path: &Path) -> HashMap<PathBuf, SearchEntry> {
        fn index_direcory(path: &Path) -> IndexDirReturn {
            let mut children_rel_path = Vec::new();

            let new_entries = Search::index_path(&path);
            for (i, _) in &new_entries {
                let parent_path = match i.parent() {
                    Some(parent_path) => parent_path,
                    None => i,
                };

                if &parent_path == &path {
                    let rel_path = match i.file_name().ok_or(i) {
                        Ok(rel_string) => PathBuf::from(rel_string.to_string_lossy().as_str()),
                        Err(_) => i.clone(),
                    };
                    children_rel_path.push(rel_path);
                };
            }

            IndexDirReturn::new(new_entries, children_rel_path)
        }

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

            let path_entry_type = SearchEntry::get_entry_type_from_path(&path);

            if matches!(path_entry_type, EntryType::Directory,) {
                let mut child_return = index_direcory(&path);

                let children = child_return.get_children();
                for rel_path_child in children {
                    search_entry.add_child(rel_path_child)
                }

                search_entries.extend(child_return.get_new_entries());
            };

            search_entries.insert(path, search_entry);
        }

        search_entries
    }
}

struct IndexDirReturn {
    new_entries: HashMap<PathBuf, SearchEntry>,
    children: Vec<PathBuf>,
}
impl IndexDirReturn {
    pub fn new(new_entries: HashMap<PathBuf, SearchEntry>, children: Vec<PathBuf>) -> Self {
        Self {
            new_entries,
            children: children,
        }
    }

    pub fn get_new_entries(&mut self) -> HashMap<PathBuf, SearchEntry> {
        take(&mut self.new_entries)
    }

    pub fn get_children(&mut self) -> Vec<PathBuf> {
        take(&mut self.children)
    }
}
