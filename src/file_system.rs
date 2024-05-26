pub mod enums;
pub mod error;

use crate::FexplorerError;

pub use enums::LinkType;
pub use error::FileSystemError;

use std::{
    fs::{self, ReadDir},
    path::{Path, PathBuf},
};

type Directory = PathBuf;
type File = PathBuf;
type Link = PathBuf;

pub fn get_entries(path: &Path) -> Result<Vec<PathBuf>, FexplorerError> {
    let mut entries: Vec<PathBuf> = Vec::new();

    for entry in read_dir(path)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        entries.push(entry.path())
    }

    Ok(entries)
}

pub fn get_entries_sorted(
    path: &Path,
) -> Result<(Vec<Directory>, Vec<File>, Vec<Link>), FexplorerError> {
    let entries = get_entries(path)?;

    sort_entries(entries)
}

pub fn sort_entries(
    entries: Vec<PathBuf>,
) -> Result<(Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>), FexplorerError> {
    let mut directories = Vec::new();
    let mut files = Vec::new();
    let mut link = Vec::new();

    for entry in entries {
        if is_directory(&entry) && !is_link(&entry) {
            directories.push(entry);
        } else if is_file(&entry) && !is_link(&entry) {
            files.push(entry);
        } else if is_link(&entry) {
            link.push(entry);
        }
    }

    Ok((directories, files, link))
}

pub fn get_rel_path(path: &Path) -> PathBuf {
    match path.file_name() {
        Some(name) => PathBuf::from(name),
        None => path.to_path_buf(),
    }
}

pub fn get_string_from_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

pub fn is_directory(path: &Path) -> bool {
    path.is_dir() && !is_link(path)
}

pub fn is_file(path: &Path) -> bool {
    path.is_file() && !is_link(path)
}

pub fn is_link(path: &Path) -> bool {
    // Check if is link
    match read_link(path) {
        Ok(_) => true,                        // Is a link
        Err(_) => false || path.is_symlink(), // Is not a link
    }
}

pub fn is_link_to_directory(path: &Path) -> bool {
    path.is_dir() && is_link(path)
}

pub fn is_link_to_file(path: &Path) -> bool {
    path.is_file() && is_link(path)
}

pub fn get_link_target(path: &Path) -> Result<PathBuf, FexplorerError> {
    match read_link(path) {
        Ok(path) => Ok(path),
        Err(e) => Err(e),
    }
}

pub fn get_link_type(path: &Path) -> Result<LinkType, FexplorerError> {
    if is_link_to_directory(path) {
        Ok(LinkType::ToDirectory(path.to_path_buf()))
    } else if is_link_to_file(path) {
        Ok(LinkType::ToFile(path.to_path_buf()))
    } else {
        Err(FexplorerError::FileSystem(FileSystemError::NotALink(
            path.to_path_buf(),
        )))
    }
}

pub fn get_path_name(path: &Path) -> String {
    match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => path.to_string_lossy().to_string(),
    }
}

fn read_dir(path: &Path) -> Result<ReadDir, FexplorerError> {
    match fs::read_dir(path) {
        Ok(read_dir) => Ok(read_dir),
        Err(e) => Err(FexplorerError::IO(e)),
    }
}

fn read_link(path: &Path) -> Result<PathBuf, FexplorerError> {
    match fs::read_link(&path) {
        Ok(path) => Ok(path),
        Err(e) => Err(FexplorerError::IO(e)),
    }
}
