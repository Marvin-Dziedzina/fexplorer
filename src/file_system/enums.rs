use std::path::PathBuf;

pub enum LinkType {
    ToFile(PathBuf),
    ToDirectory(PathBuf),
}
