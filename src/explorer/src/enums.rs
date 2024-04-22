use std::fmt;

pub enum EntryType {
    Directory,
    File,
    Link,
    Unknown,
}
impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntryType::Directory => write!(f, "Directory"),
            EntryType::File => write!(f, "File"),
            EntryType::Link => write!(f, "Link"),
            _ => write!(f, "Unknown"),
        }
    }
}
