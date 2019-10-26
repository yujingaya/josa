use std::{fmt, error, result};

use hangul::ParseSyllableError;

/// Error type for appending josa to a string. Occurs when the string is empty, 
/// or does not end with Hangul Syllable.
#[derive(Debug)]
pub enum Error {
  EmptyStr,
  ParseSyllable(char),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Error::EmptyStr => "Empty string given to josa selector".to_owned(),
        Error::ParseSyllable(c) => format!("{} is not a Hangul Syllable", c),
      }
    )
  }
}

impl error::Error for Error {}

impl From<ParseSyllableError> for Error {
  fn from(e: ParseSyllableError) -> Error {
    Error::ParseSyllable(e.0)
  }
}

/// A specialized [`Result`] type for josa appending operations.
/// 
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
pub type Result<T> = result::Result<T, Error>;
