/* use std::fs;
use std::path::{Path, PathBuf};

use crate::entries::link::LinkType;
use crate::entries::{Directory, File, Link, PathTrait};

use super::Error;

#[derive(Debug)]
pub struct Entries {
    directories: Vec<Directory>,
    files: Vec<File>,
    links: Vec<Link>,
}
impl Entries {
    pub fn new(path: &Path) -> Result<Self, Error> {
        match path.try_exists() {
            Ok(_) => (),
            Err(_) => {
                return Err(Error::PathDoesNotExist(format!(
                    "'{}' does not exist!",
                    path.to_string_lossy().to_string()
                )))
            }
        };

        let (directory_paths, file_paths, link_paths) = match Self::search_entries(path) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };

        Ok(Self {
            directories: Self::convert_directory_paths_to_directory(directory_paths),
            files: Self::convert_file_paths_to_file(file_paths),
            links: Self::convert_link_paths_to_link(link_paths),
        })
    }

    fn convert_directory_paths_to_directory(paths: Vec<PathBuf>) -> Vec<Directory> {
        let mut directories: Vec<Directory> = Vec::new();

        for dir in paths {
            let children = match Self::get_children(&dir) {
                Ok(children) => children,
                Err(_) => continue,
            };

            let directory = match Directory::new(&dir, Some(children)) {
                Ok(directory) => directory,
                Err(_) => continue,
            };

            directories.push(directory);
        }

        directories
    }

    fn convert_file_paths_to_file(paths: Vec<PathBuf>) -> Vec<File> {
        let mut files: Vec<File> = Vec::new();

        for file in paths {
            let file = match File::new(&file) {
                Ok(file) => file,
                Err(_) => continue,
            };

            files.push(file);
        }

        files
    }

    fn convert_link_paths_to_link(paths: Vec<LinkType>) -> Vec<Link> {
        let mut links = Vec::new();

        for link_type in paths {
            let link = match Link::new(&link_type.get_path()) {
                Ok(link) => link,
                Err(_) => continue,
            };

            links.push(link);
        }

        links
    }

    fn search_entries(path: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>, Vec<LinkType>), Error> {
        let dirs = match fs::read_dir(path) {
            Ok(read_dir) => read_dir,
            Err(e) => return Err(Error::IO(e)),
        };

        let mut directories: Vec<PathBuf> = Vec::new();
        let mut files: Vec<PathBuf> = Vec::new();
        let mut links: Vec<LinkType> = Vec::new();

        for entry in dirs {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let entry_path = entry.path();

            let is_directory = entry_path.is_dir();
            let is_file = entry_path.is_file();
            // Check if is link
            let is_link = match fs::read_link(&path) {
                Ok(_) => true,   // Is a link
                Err(_) => false, // Is not a link
            };

            if is_directory && !is_link {
                directories.push(entry_path);
            } else if is_file && !is_link {
                files.push(entry_path);
            } else if is_link && is_directory {
                links.push(LinkType::Directory(entry_path));
            } else if is_link && is_file {
                links.push(LinkType::File(entry_path));
            }
        }

        Ok((directories, files, links))
    }

    pub fn get_children(path: &Path) -> Result<Vec<PathBuf>, Error> {
        let entries_object = Entries::new(path)?;

        let mut entries: Vec<PathBuf> = Vec::new();

        let (directories, files, links) = entries_object.get_entries();
        for dir in directories {
            entries.push(dir.get_path().to_path_buf());
        }

        for file in files {
            entries.push(file.get_path().to_path_buf());
        }

        for link in links {
            entries.push(link.get_path().to_path_buf());
        }

        Ok(entries)
    }

    pub fn get_entries(&self) -> (Vec<Directory>, Vec<File>, Vec<Link>) {
        (self.directories, self.files, self.links)
    }

    pub fn get_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        let (directories, files, links) = self.get_entries();
        for dir in directories {
            paths.push(dir.get_path().to_path_buf());
        }

        for file in files {
            paths.push(file.get_path().to_path_buf());
        }

        for link in links {
            paths.push(link.get_path().to_path_buf());
        }

        paths
    }
}
 */
