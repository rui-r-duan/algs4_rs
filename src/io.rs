use crate::scanner::Scanner;
use std::io;
use std::io::Read;
use std::io::StdinLock;

pub struct StdIn {
    scanner: Scanner<StdinLock<'static>>,
}

impl StdIn {
    pub fn new() -> Self {
        StdIn {
            scanner: Scanner::new(io::stdin().lock()),
        }
    }

    pub fn is_empty(&mut self) -> bool {
        match self.scanner.has_next() {
            Ok(b) => !b,
            Err(_) => true,
        }
    }

    pub fn read_i32(&mut self) -> io::Result<i32> {
        self.scanner.next_i32()
    }

    /// Read all 32-bit integers (i32) from stdin, consuming all the
    /// content in stdin.
    pub fn read_all_i32() -> io::Result<Vec<i32>> {
        let mut stdin = io::stdin();
        let mut all = String::new();
        stdin.read_to_string(&mut all)?;
        let mut list = Vec::new();
        for t in all.split_ascii_whitespace() {
            match t.parse::<i32>() {
                Ok(n) => {
                    list.push(n);
                }
                Err(e) => {
                    eprintln!("got error: {}", e);
                    return Err(io::Error::from(io::ErrorKind::InvalidData));
                }
            }
        }
        Ok(list)
    }

    /// Read all 32-bit integers (i32) from stdin using `Scanner`,
    /// consuming all the content in stdin, reading the content in a
    /// token-by-token streaming mode.
    pub fn read_all_i32_streaming(&mut self) -> io::Result<Vec<i32>> {
        let mut list = Vec::new();
        loop {
            let hasnext = self.scanner.has_next()?;
            if !hasnext {
                break;
            }

            let t = self.scanner.next_i32()?;
            list.push(t);
        }
        Ok(list)
    }
}

pub struct In {
    file_path: String,
}

impl In {
    pub fn new(path: &str) -> Self {
        In {
            file_path: path.to_string(),
        }
    }

    /// Read all 32-bit integers (i32) from stdin, consuming all the
    /// content in stdin.
    pub fn read_all_i32(&self) -> io::Result<Vec<i32>> {
        let mut f = std::fs::File::open(self.file_path.as_str())?;
        let mut all = String::new();
        f.read_to_string(&mut all)?;
        let mut list = Vec::new();
        for t in all.split_ascii_whitespace() {
            match t.parse::<i32>() {
                Ok(n) => {
                    list.push(n);
                }
                Err(e) => {
                    return Err(io::Error::from(io::ErrorKind::InvalidData));
                }
            }
        }
        Ok(list)
    }
}
