#[derive(Debug)]
pub enum Error {
    PathDoesNotExist(String),
    FaultyName(String),
    ConversionFailure(String),
    IO(std::io::Error),
}
