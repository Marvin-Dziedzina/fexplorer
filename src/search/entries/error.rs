use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    InvalidEntryType(String),
    IO(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", &self)
    }
}
