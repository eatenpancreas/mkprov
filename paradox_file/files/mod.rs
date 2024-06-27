mod pdx;

pub use pdx::*;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

pub trait AsFilename: Debug {
  fn as_filename(&self, dir: &PathBuf) -> Option<String>;
}

impl AsFilename for &str {
  fn as_filename(&self, dir: &PathBuf) -> Option<String> {
    Some(format!("{}/{self}", dir.to_str()?))
  }
}

#[derive(Debug)]
pub(crate) struct Filename(String);

impl AsFilename for Filename {
  fn as_filename(&self, dir: &PathBuf) -> Option<String> {
    Some(format!("{}/{}", dir.to_str()?, self.0))
  }
}

pub(crate) struct LocalFile {
  path: String
}

impl LocalFile {
  pub(crate) fn get_filename<T: AsFilename>(base_path: &str, sub_directory: &str, filename: &T) -> Option<Filename> {
    Self::get_name_inner(&mut PathBuf::from(base_path), sub_directory, filename)
        .and_then(|f| Some(Filename(f)))
  }

  fn get_name_inner<T: AsFilename>(path: &mut PathBuf, sub_directory: &str, filename: &T) -> Option<String> {
    path.push(sub_directory);

    filename.as_filename(&path)
  }

  pub(crate) fn get_file<T: AsFilename>(base_path: &str, sub_directory: &str, filename: &T) -> Option<LocalFile> {
    let mut pb = &mut PathBuf::from(base_path);
    let name = Self::get_name_inner(&mut pb, sub_directory, filename)?;

    pb.push(name);
    Some(LocalFile {
      path: pb.to_str()?.to_string()
    })
  }

  pub fn get_contents(&self) -> Option<String> {
    Some(String::from_utf8(fs::read(&self.path).unwrap()).unwrap())
  }

  pub fn write_contents(&self, contents: &String) -> bool {
    fs::write(&self.path, contents).is_ok()
  }
}
