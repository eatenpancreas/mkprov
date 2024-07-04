use std::{error, io};

pub trait IntoResult<T> {
  fn into_result<E>(self, error: E) -> io::Result<T> where 
    E: Into<Box<dyn error::Error + Send + Sync>>;
}

pub trait IntoError {
  fn into_error<E>(self, error: E) -> io::Error where
    E: Into<Box<dyn error::Error + Send + Sync>>;
}

impl <T> IntoResult<T> for io::ErrorKind {
  fn into_result<E>(self, error: E) -> io::Result<T> where
    E: Into<Box<dyn error::Error + Send + Sync>>
  {
    Err(self.into_error(error))
  }
}

impl IntoError for io::ErrorKind {
  fn into_error<E>(self, error: E) -> io::Error where
    E: Into<Box<dyn error::Error + Send + Sync>>
  {
    io::Error::new(self, error)
  }
}