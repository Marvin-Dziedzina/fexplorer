use std::{path::PathBuf, thread::JoinHandle};

use super::search_entry::SearchEntry;

pub struct SearchEntryThreadData {
    path: PathBuf,
    children_join_handle: JoinHandle<Vec<SearchEntry>>,
}
impl SearchEntryThreadData {
    pub fn new(path: PathBuf, children_join_handle: JoinHandle<Vec<SearchEntry>>) -> Self {
        Self {
            path,
            children_join_handle,
        }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn get_children_join_handle(self) -> JoinHandle<Vec<SearchEntry>> {
        self.children_join_handle
    }
}
