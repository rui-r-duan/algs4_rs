use crate::primitive::{PrimFloat, PrimInt};
use crate::scanner::Scanner;
use std::fs::File;
use std::io::BufReader;
use std::io::StdinLock;
use std::io::{self, BufRead};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// General Input (stdin, file, socket, etc.).
pub struct In<B: BufRead> {
    scanner: Scanner<B>,
}

impl<B: BufRead> In<B> {
    /// Creates a new instance of BaseInput.
    pub fn new(bufread: B) -> Self {
        In {
            scanner: Scanner::new(bufread),
        }
    }

    /// Returns true if the input stream has more data, returns false otherwise.
    pub fn is_empty(&mut self) -> bool {
        match self.scanner.has_next() {
            Ok(b) => !b,
            Err(_) => true,
        }
    }

    /// Reads an integer from the input stream.
    ///
    /// The integer type is one of `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`,
    /// `u64`, `u128`, or `usize`.
    ///
    /// # Errors
    ///
    /// Same as `Scanner::next_int`.
    pub fn read_int<T>(&mut self) -> io::Result<T>
    where
        T: PrimInt + FromStr,
    {
        self.scanner.next_int::<T>()
    }

    /// Reads a floating point number from the input stream.
    ///
    /// The integer type is one of `f32` or `f64`.
    ///
    /// # Errors
    ///
    /// Same as `Scanner::next_float`.
    pub fn read_float<T>(&mut self) -> io::Result<T>
    where
        T: PrimFloat + FromStr,
    {
        self.scanner.next_float::<T>()
    }

    /// Reads all integers from the input stream using the internal scanner, consuming all the
    /// content in the input stream, reading the content in a token-by-token streaming mode.
    ///
    /// # Errors
    ///
    /// Same as `Scanner::next_int`.
    pub fn read_all_ints<T>(&mut self) -> io::Result<Vec<T>>
    where
        T: PrimInt + FromStr,
    {
        let mut list = Vec::new();
        loop {
            if !self.scanner.has_next()? {
                break;
            }
            list.push(self.scanner.next_int::<T>()?);
        }
        Ok(list)
    }

    /// Read a string token from the input stream.
    ///
    /// # Errors
    ///
    /// Same as `Scanner::next_token`.
    pub fn read_string(&mut self) -> io::Result<String> {
        self.scanner.next_token()
    }

    /// Reads all string tokens from the input stream using the internal scanner, consuming all the
    /// content in the input stream, reading the content in a token-by-token streaming mode.
    ///
    /// # Errors
    ///
    /// Same as `Scanner::next_token`.
    pub fn read_all_strings(&mut self) -> io::Result<Vec<String>> {
        let mut list = Vec::new();
        loop {
            if !self.scanner.has_next()? {
                break;
            }
            list.push(self.scanner.next_token()?);
        }
        Ok(list)
    }
}

/// Standard input of this library.
pub struct StdIn(In<StdinLock<'static>>);

impl Deref for StdIn {
    type Target = In<StdinLock<'static>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StdIn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl StdIn {
    pub fn new() -> Self {
        StdIn(In::new(io::stdin().lock()))
    }
}

impl Default for StdIn {
    fn default() -> Self {
        Self::new()
    }
}

/// File input.
pub struct FileIn(In<BufReader<File>>);

impl Deref for FileIn {
    type Target = In<BufReader<File>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FileIn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FileIn {
    /// Creates a new instance of In.
    pub fn new(path: &str) -> io::Result<Self> {
        let f = std::fs::File::open(path)?;
        Ok(FileIn(In::new(BufReader::new(f))))
    }
}
