use core::result;

use std::{error::Error as StdError, fmt, io};

use notify;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
pub enum ErrorCode {
  Message(Box<String>),
}

#[derive(Debug)]
struct ErrorImpl {
  code: ErrorCode,
}

impl ErrorImpl {
  pub fn new(code: ErrorCode) -> Self {
    Self { code }
  }
}

#[derive(Debug)]
pub struct Error {
  err: ErrorImpl,
}

impl Error {
  pub fn new(error_message: String) -> Self {
    Self {
      err: ErrorImpl::new(
        ErrorCode::Message(Box::new(error_message))
      )
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let error_message = match &self.err.code {
      ErrorCode::Message(message) => String::from(message.as_ref())
    };

    write!(f, "{}", error_message)
  }
}

impl StdError for Error {
  fn cause(&self) -> Option<&dyn StdError> {
    None
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Error::new(err.to_string())
  }
}

impl From<notify::Error> for Error {
  fn from(err: notify::Error) -> Self {
    Error::new(err.to_string())
  }
}

impl From<SerdeJsonError> for Error {
  fn from(err: SerdeJsonError) -> Self {
    Error::new(err.to_string())
  }
}

pub type Result<T> = result::Result<T, Error>;
