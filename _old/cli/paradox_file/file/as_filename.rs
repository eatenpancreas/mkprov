use std::fmt::{Debug, Display};
use std::io;
use std::path::PathBuf;
use crate::IntoResult;

pub trait AsFilename: Debug {
  fn as_filename(&self, dir: &PathBuf) -> io::Result<String>;
}

impl AsFilename for &str {
  fn as_filename(&self, dir: &PathBuf) -> io::Result<String> {
    as_filename_inner(self, dir)
  }
}

#[derive(Debug)]
pub(crate) struct Filename(pub(crate) String);

impl AsFilename for Filename {
  fn as_filename(&self, dir: &PathBuf) -> io::Result<String> {
    as_filename_inner(&self.0, dir)
  }
}

fn as_filename_inner<T: Display>(from: &T, dir: &PathBuf) -> io::Result<String> {
  match dir.to_str() {
    None => io::ErrorKind::Unsupported.into_result("Could not parse filename"),
    Some(dir) => Ok(format!("{dir}/{from}"))
  }
}