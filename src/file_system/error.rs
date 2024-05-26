use std::path::PathBuf;

#[derive(Debug)]
pub enum FileSystemError {
    NotADirectory(PathBuf),
    NotAFile(PathBuf),
    NotALink(PathBuf),
}
