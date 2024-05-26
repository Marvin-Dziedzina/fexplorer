use std::{fmt::Display, io};

use crate::file_system::FileSystemError;

#[derive(Debug)]
pub enum FexplorerError {
    IO(io::Error),
    IOVec(Vec<io::Error>),
    PathDoesNotExist(String),
    FileSystem(FileSystemError),
    OpenError(opener::OpenError),
}

impl Display for FexplorerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", &self)
    }
}
