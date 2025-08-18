/// Error type used for this algs4 library
#[derive(Debug)]
pub enum Algs4Error {
    InvalidArgument(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for Algs4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algs4Error::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            Algs4Error::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for Algs4Error {}

/// Convert std::io::Error to Algs4Error
impl From<std::io::Error> for Algs4Error {
    fn from(err: std::io::Error) -> Self {
        Algs4Error::IoError(err)
    }
}
