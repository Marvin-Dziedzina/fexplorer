#[derive(Debug)]
pub enum Error {
    Generic(String),
    PathDoesNotExist(String),
    FaultyName(String),
    ConversionFailure(String),
    IO(std::io::Error),
}
