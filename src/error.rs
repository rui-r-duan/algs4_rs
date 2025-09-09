//! Error types of algs4_rs.

use std::{error, fmt, io};

/// Error type used for this algs4 library
#[derive(Debug)]
pub enum Algs4Error {
    InvalidArgument(String),
    IoError(io::Error),
}

impl fmt::Display for Algs4Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Algs4Error::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            Algs4Error::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl error::Error for Algs4Error {}

/// Convert `io::Error` to `Algs4Error`
impl From<io::Error> for Algs4Error {
    fn from(err: io::Error) -> Self {
        Algs4Error::IoError(err)
    }
}

/// Error type used to indicate an invalid argument
#[derive(Debug)]
pub struct InvalidArgument(pub String);

impl fmt::Display for InvalidArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid argument: {}", self.0)
    }
}

impl error::Error for InvalidArgument {}

/// Convert `InvalidArgument` to `Algs4Error`
impl From<InvalidArgument> for Algs4Error {
    fn from(err: InvalidArgument) -> Self {
        Algs4Error::InvalidArgument(err.0)
    }
}
