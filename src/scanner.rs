//! `Scanner` takes a `BufRead` for a *text input*, and yields the next entity until EOF is reached.
//!
//! A valid text input consists of delimiters and tokens.  The delimiters are ASCII whitespaces
//! (U+0009 TAB, U+000A LF, U+000C FF, U+000D CR, or U+0020 SPACE).  The tokens consist of the other
//! UTF-8 characters.
//!
//! It operates in two modes, and the two modes' methods can be used together in any order.
//!
//! # Mode 1: token-by-token (delimiter: ASCII whitespaces)
//! methods:
//! - `has_next`
//! - `next`
//! - `next_i32`
//! - `next_i64`
//! - `next_f64`
//! - `next_bool`
//!
//! # Mode 2: line-by-line (delimiter: U+000A LF)
//! methods:
//! - `has_next_line`
//! - `next_line`

use std::io;
use std::io::BufRead;

pub struct Scanner<B: BufRead> {
    bufread: B,
    buf: Vec<u8>,       // buffer for bytes read from BufRead
    consume_pos: usize, // the starting point in buf for the next consume

    token_peek_pos: usize, // the starting point in buf for the next token peeking
    next_token: Option<String>,
    token_peeked: bool,

    line_peek_pos: usize, // the starting point in buf for the next line peeking
    next_line: Option<String>,
    line_peeked: bool,
}

impl<B: BufRead> Scanner<B> {
    pub fn new(bufread: B) -> Self {
        Scanner {
            bufread,
            buf: Vec::new(),
            consume_pos: 0,
            token_peek_pos: 0, // invariant: token_peek_pos >= consume_pos
            next_token: None,
            token_peeked: false,
            line_peek_pos: 0, // invariant: line_peek_pos >= consume_pos
            next_line: None,
            line_peeked: false,
        }
    }

    /// Checks if there is next token available.
    ///
    /// A token is a sequence of non-ascii-whitespace UTF-8 characters.
    ///
    /// If such a token is found, return `Ok(true)`, otherwise, return `Ok(false)`.
    ///
    /// # Errors
    ///
    /// If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    /// `Interrupted`, because it is handled (ignored) in this method.
    pub fn has_next(&mut self) -> io::Result<bool> {
        match self.peek_next() {
            Ok(_) => Ok(true),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(false),
                std::io::ErrorKind::InvalidData => Ok(false),
                _ => Err(e),
            },
        }
    }

    /// Checks if there is next line available.
    ///
    /// A line is a sequence of UTF-8 characters.
    ///
    /// If such a line is found, return `Ok(true)`, otherwise, return `Ok(false)`.
    ///
    /// # Errors
    ///
    /// If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    /// `Interrupted`, because it is handled (ignored) in this method.
    pub fn has_next_line(&mut self) -> io::Result<bool> {
        match self.peek_next_line() {
            Ok(_) => Ok(true),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(false),
                std::io::ErrorKind::InvalidData => Ok(false),
                _ => Err(e),
            },
        }
    }

    // Reads new data from the underlying BufRead, consumes the new data, and extends the internal
    // buffer, returns the length of the new data.
    //
    // If any IO Errors is encountered, return it as `Err`.
    fn read_new_data(&mut self) -> io::Result<usize> {
        let available = match self.bufread.fill_buf() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        self.buf.extend_from_slice(available);
        let len = available.len();
        self.bufread.consume(len);
        Ok(len)
    }

    // Scans internal buffer to find the first target, extend buffer if necessary by reading more
    // data from the underlying BufRead, until the first target is found, or EOF is reached.
    //
    // Returns the index of the found delim in the form `Ok(Some(index))`, if not found, returns
    // `Ok(None)`.
    //
    // If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    // `Interrupted`, because it is handled (ignored) in this method.
    fn peek_until<P>(&mut self, predicate: P, begin: usize) -> io::Result<Option<usize>>
    where
        P: Fn(u8) -> bool,
    {
        match position(&predicate, &self.buf, begin) {
            Some(index) => Ok(Some(index)),
            None => {
                let mut pos = self.buf.len();
                loop {
                    let (done, peeked_size) = {
                        let new_len = match self.read_new_data() {
                            Ok(n) => n,
                            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                            Err(e) => return Err(e),
                        };
                        match position(&predicate, &self.buf, pos) {
                            Some(index) => (true, index - pos + 1),
                            None => (false, new_len),
                        }
                    };
                    pos += peeked_size;
                    if done {
                        return Ok(Some(pos - 1));
                    } else if peeked_size == 0 {
                        return Ok(None);
                    }
                }
            }
        }
    }

    fn drop_consumed_part(&mut self) {
        self.buf.drain(..self.consume_pos);
        self.token_peek_pos -= self.consume_pos;
        self.line_peek_pos -= self.consume_pos;
        self.consume_pos = 0;
    }

    // Peeks the next token.  Read until the end of the next token or the end of the input stream.
    //
    // If no next token is found, return IO Error `NotFound`.
    //
    // If the next token has any invalid UTF-8 character, return IO Error `InvalidData`.
    //
    // If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    // `Interrupted`, because it is handled (ignored) in this method.
    fn peek_next(&mut self) -> io::Result<()> {
        if self.next_token.is_some() {
            return Ok(());
        }
        self.token_peeked = true;

        // Find the first non-whitespace.
        let i_result = self.peek_until(|x: u8| !x.is_ascii_whitespace(), self.token_peek_pos);
        if i_result.is_err() {
            return Err(i_result.unwrap_err());
        }
        let i_opt = i_result.unwrap();
        if i_opt.is_none() {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }
        let i = i_opt.unwrap();

        // Find the next whitespace
        let j_result = self.peek_until(|x: u8| x.is_ascii_whitespace(), i + 1);
        if j_result.is_err() {
            return Err(j_result.unwrap_err());
        }
        let j_opt = j_result.unwrap();
        let j = if j_opt.is_none() {
            self.buf.len()
        } else {
            j_opt.unwrap()
        };

        self.token_peek_pos = j;
        match std::str::from_utf8(&self.buf[i..j]) {
            Ok(s) => {
                self.token_peek_pos = j;
                self.next_token = Some(s.to_string());
                Ok(())
            }
            Err(_e) => Err(io::Error::from(io::ErrorKind::InvalidData)),
        }
    }

    // Peeks the next line.  Read until the next Line Feed or the end of the input stream.
    //
    // If the next line is found, it is stored in `self.next_line`.
    // The line separator is included if it is found.
    //
    // If no next line is found, return IO Error `NotFound`.
    //
    // If the next line has any invalid UTF-8 character, return IO Error `InvalidData`.
    //
    // If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    // `Interrupted`, because it is handled (ignored) in this method.
    fn peek_next_line(&mut self) -> io::Result<()> {
        if self.next_line.is_some() {
            return Ok(());
        }
        self.line_peeked = true;

        // Find the next Line Feed
        let j_result = self.peek_until(|x: u8| x == b'\n', self.line_peek_pos);
        if j_result.is_err() {
            return Err(j_result.unwrap_err());
        }
        let j_opt = j_result.unwrap();
        let new_line_peek_pos = if j_opt.is_none() {
            self.buf.len()
        } else {
            j_opt.unwrap() + 1
        };

        if self.line_peek_pos == new_line_peek_pos {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }

        match std::str::from_utf8(&self.buf[self.line_peek_pos..new_line_peek_pos]) {
            Ok(s) => {
                self.line_peek_pos = new_line_peek_pos;
                self.next_line = Some(s.to_string());
                Ok(())
            }
            Err(_e) => Err(io::Error::from(io::ErrorKind::InvalidData)),
        }
    }

    fn mark_token_consumed(&mut self) {
        self.consume_pos = self.token_peek_pos;
        self.token_peeked = false;
        self.next_token = None;

        self.line_peek_pos = self.consume_pos;
        self.next_line = None;
        self.line_peeked = false;

        self.drop_consumed_part();
    }

    fn mark_line_consumed(&mut self) {
        self.consume_pos = self.line_peek_pos;
        self.line_peeked = false;
        self.next_line = None;

        self.token_peek_pos = self.consume_pos;
        self.next_token = None;
        self.token_peeked = false;

        self.drop_consumed_part();
    }

    /// Reads the next token as a `String`.
    ///
    /// A token is a sequence of non-ascii-whitespace UTF-8 characters.
    ///
    /// # Errors
    ///
    /// If no such token is found, return IO Error `NotFound`.
    ///
    /// If the next token has any invalid UTF-8 character, return IO Error `InvalidData`.
    ///
    /// If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    /// `Interrupted`, because it is handled (ignored) in this method.
    ///
    /// If any Error (including `NotFound`) is returned, then the input stream's cursor is not
    /// changed, which means that if the client calls another `next_*` method immediately, the next
    /// entity (if fetched successfully) may contain the characters of this invalid token.  For
    /// example, in the upcoming input stream all characters are whitespaces, so no valid next token
    /// is found, however, if you call `next_line`, these whitespaces will be included in the next
    /// line because they form a valid line.
    pub fn next(&mut self) -> io::Result<String> {
        if !self.token_peeked {
            self.peek_next()?;
        }
        if self.next_token.is_none() {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        } else {
            let result = self.next_token.as_ref().unwrap().clone();
            self.mark_token_consumed();
            Ok(result)
        }
    }

    /// Reads the next token as an `i32`.
    ///
    /// # Errors
    ///
    /// If no such i32 is found, return IO Error `NotFound`.
    ///
    /// If the next token is not a valid i32, return IO Error `InvalidData`.
    ///
    /// If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    /// `Interrupted`, because it is handled (ignored) in this method.
    ///
    /// If any Error (including `NotFound`) is returned, then the input stream's cursor is not
    /// changed, which means that if the client calls another `next_*` method immediately, the next
    /// entity (if fetched successfully) contains the characters of this invalid token.  For
    /// example, the next token is not a valid i32, but the token is a valid UTF-8 string, then it
    /// may be a valid bool for the next call to `next_bool`, or the next call to `next_line` will
    /// include this token in the line.
    pub fn next_i32(&mut self) -> io::Result<i32> {
        if !self.token_peeked {
            self.peek_next()?;
        }
        if self.next_token.is_none() {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        } else {
            let s = self.next_token.as_ref().unwrap();
            match s.parse::<i32>() {
                Ok(v) => {
                    self.mark_token_consumed();
                    Ok(v)
                }
                Err(_e) => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
            }
        }
    }

    /// Reads the next token as an `i64`.
    ///
    /// # Errors
    ///
    /// If no such i64 is found, return IO Error `NotFound`.
    ///
    /// If the next token is not a valid i64, return IO Error `InvalidData`.
    ///
    /// If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    /// `Interrupted`, because it is handled (ignored) in this method.
    ///
    /// If any Error (including `NotFound`) is returned, then the input stream's cursor is not
    /// changed, which means that if the client calls another `next_*` method immediately, the next
    /// entity (if fetched successfully) contains the characters of this invalid token.  For
    /// example, the next token is not a valid i64, but the token is a valid UTF-8 string, then it
    /// may be a valid bool for the next call to `next_bool`, or the next call to `next_line` will
    /// include this token in the line.
    pub fn next_i64(&mut self) -> io::Result<i64> {
        if !self.token_peeked {
            self.peek_next()?;
        }
        if self.next_token.is_none() {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        } else {
            let s = self.next_token.as_ref().unwrap();
            match s.parse::<i64>() {
                Ok(v) => {
                    self.mark_token_consumed();
                    Ok(v)
                }
                Err(_e) => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
            }
        }
    }

    /// Reads the next token as an `f64`.
    ///
    /// # Errors
    ///
    /// If no such f64 is found, return IO Error `NotFound`.
    ///
    /// If the next token is not a valid f64, return IO Error `InvalidData`.
    ///
    /// If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    /// `Interrupted`, because it is handled (ignored) in this method.
    ///
    /// If any Error (including `NotFound`) is returned, then the input stream's cursor is not
    /// changed, which means that if the client calls another `next_*` method immediately, the next
    /// entity (if fetched successfully) contains the characters of this invalid token.  For
    /// example, the next token is not a valid f64, but the token is a valid UTF-8 string, then it
    /// may be a valid bool for the next call to `next_bool`, or the next call to `next_line` will
    /// include this token in the line.
    pub fn next_f64(&mut self) -> io::Result<f64> {
        if !self.token_peeked {
            self.peek_next()?;
        }
        if self.next_token.is_none() {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        } else {
            let s = self.next_token.as_ref().unwrap();
            match s.parse::<f64>() {
                Ok(v) => {
                    self.mark_token_consumed();
                    Ok(v)
                }
                Err(_e) => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
            }
        }
    }

    /// Reads the next token as a `bool`.
    ///
    /// # Errors
    ///
    /// If no such bool is found, return IO Error `NotFound`.
    ///
    /// If the next token is not a valid bool, return IO Error `InvalidData`.
    ///
    /// If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    /// `Interrupted`, because it is handled (ignored) in this method.
    ///
    /// If any Error (including `NotFound`) is returned, then the input stream's cursor is not
    /// changed, which means that if the client calls another `next_*` method immediately, the next
    /// entity (if fetched successfully) contains the characters of this invalid token.  For
    /// example, the next token is not a valid bool, but the token is a valid UTF-8 string, then it
    /// may be a valid i32 for the next call to `next_i32`, or the next call to `next_line` will
    /// include this token in the line.
    pub fn next_bool(&mut self) -> io::Result<bool> {
        if !self.token_peeked {
            self.peek_next()?;
        }
        if self.next_token.is_none() {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        } else {
            let token = self
                .next_token
                .as_ref()
                .unwrap()
                .clone()
                .to_ascii_lowercase();
            let result = match token.as_str() {
                "true" | "1" => {
                    self.mark_token_consumed();
                    Ok(true)
                }
                "false" | "0" => {
                    self.mark_token_consumed();
                    Ok(false)
                }
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "attempts to read a 'bool' value from the input stream, but the next token is \"{}\"",
                        token
                    ),
                )),
            };
            result
        }
    }

    /// Reads until the next Line Feed or the end of the input stream, returns the line string
    /// disgarging the line separator ('\n' on Unix-like OS, "\r\n" on Windows) if any.
    ///
    /// # Errors
    ///
    /// If no next line is found (no more input data), return IO Error `NotFound`.
    ///
    /// if the next line has any invalid UTF-8 character, return IO Error `InvalidData`.
    ///
    /// If any IO Errors is encountered, return it as `Err`.  This method does not return IO Error
    /// `Interrupted`, because it is handled (ignored) in this method.
    ///
    /// If any Error (including `NotFound`) is returned, then the input stream's cursor is not
    /// changed.  However, there is no point calling any other `next_*` afterwards because they will
    /// fail.  If the next line is `NotFound`, it means there is no more character.  If the next
    /// line contains any invalid UTF-8 character, then any other `next_*` method will also report
    /// invalid UTF-8 error.  If other IO error happened, then it is likely that any other `next_*`
    /// method also runs into an IO error.
    pub fn next_line(&mut self) -> io::Result<String> {
        if !self.line_peeked {
            self.peek_next_line()?;
        }
        if self.next_line.is_none() {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        } else {
            let mut result = self.next_line.as_ref().unwrap().clone();
            if result.ends_with('\n') {
                result.pop();
                if result.ends_with('\r') {
                    result.pop();
                }
            }
            self.mark_line_consumed();
            Ok(result)
        }
    }
}

// Finds the target in buf starting at position `begin`, returns the
// target's index in `buf`.  The target must satisfy the predicate.
//
// If `begin >= buf.len()`, returns `None`.
//
// If no such target is found, returns `None`.
fn position<P>(predicate: P, buf: &[u8], begin: usize) -> Option<usize>
where
    P: Fn(u8) -> bool,
{
    if begin >= buf.len() {
        return None;
    }
    for (i, &x) in buf[begin..].iter().enumerate() {
        if predicate(x) {
            return Some(i + begin);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_until_1() {
        let input_data = "10 20 30 40\nhello world\n50";
        let cursor = std::io::Cursor::new(input_data);
        let mut scanner = Scanner::new(cursor);
        let r = scanner
            .peek_until(|x: u8| !x.is_ascii_whitespace(), 0)
            .expect("cannot fail");
        assert_eq!(r, Some(0));
    }

    #[test]
    fn test_peek_until_2() {
        let input_data = "10 20 30 40\nhello world\n50";
        let cursor = std::io::Cursor::new(input_data);
        let mut scanner = Scanner::new(cursor);
        let r = scanner
            .peek_until(|x: u8| x.is_ascii_whitespace(), 0)
            .expect("cannot fail");
        assert_eq!(r, Some(2));
    }

    #[test]
    fn test_peek_until_3() {
        let input_data = "10 20 30 40\nhello world\n50";
        let cursor = std::io::Cursor::new(input_data);
        let mut scanner = Scanner::new(cursor);
        let r = scanner
            .peek_until(|x: u8| x == b'\n', 0)
            .expect("cannot fail");
        assert_eq!(r, Some(11));
    }

    #[test]
    fn test_peek_until_4() {
        let input_data = "";
        let cursor = std::io::Cursor::new(input_data);
        let mut scanner = Scanner::new(cursor);
        let r = scanner
            .peek_until(|x: u8| x == b'\n', 0)
            .expect("cannot fail");
        assert_eq!(r, None);
    }

    #[test]
    fn test_scanner_ok_cases() {
        let input_data = "10 20 30 40\nhello world\n50";
        let cursor = std::io::Cursor::new(input_data);
        let mut scanner = Scanner::new(cursor);

        println!("--- Testing Token-by-Token Scanning ---");
        let r = scanner.next_i32();
        // Consumes "10"
        assert!(r.is_ok());
        let num = r.unwrap();
        assert_eq!(num, 10);

        // The line is now "20 30 40\n". The next call should get the rest of it.
        let r = scanner.next_line();
        // Consumes " 20 30 40" and the newline.
        assert!(r.is_ok());
        let line = r.unwrap();
        assert_eq!(line, " 20 30 40");

        println!("\n--- Testing has_next() ---");
        // `has_next()` should see the next line ("hello world") is available
        let hasnext = scanner.has_next();
        assert!(hasnext.is_ok());
        assert!(hasnext.unwrap());
        let r = scanner.next();
        assert!(r.is_ok());
        let s = r.unwrap();
        assert_eq!(s, "hello");

        println!("\n--- Testing next_bool() and remaining tokens ---");
        let input_with_bools = "true false\nfinal_token";
        let cursor_bools = std::io::Cursor::new(input_with_bools);
        let mut bool_scanner = Scanner::new(cursor_bools);

        let r = bool_scanner.next_bool();
        // Consumes "true"
        assert!(r.is_ok());
        let b = r.unwrap();
        assert!(b);

        let r = bool_scanner.next();
        // Consumes "false"
        assert!(r.is_ok());
        let s = r.unwrap();
        assert_eq!(s, "false");

        let r = bool_scanner.next_line();
        // Consumes the rest of the first line, including the Line Feed
        assert!(r.is_ok());
        let line = r.unwrap();
        assert_eq!(line, "");

        let r = bool_scanner.next_line();
        // Consumes the second line
        assert!(r.is_ok());
        let line = r.unwrap();
        assert_eq!(line, "final_token");
    }
}
