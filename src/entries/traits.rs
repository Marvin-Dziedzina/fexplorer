use std::{fs, io, path::PathBuf};

pub trait PathTrait {
    fn get_path(&self) -> &Box<PathBuf>;

    fn get_metadata(&self) -> Result<fs::Metadata, io::Error>;

    fn get_name(&self) -> String;

    fn get_rel_path(&self) -> PathBuf {
        PathBuf::from(self.get_name())
    }
}
