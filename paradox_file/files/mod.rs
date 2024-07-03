mod pdx;
mod localisation;
mod as_filename;
mod definition;

pub use pdx::*;
pub use as_filename::*;
pub use localisation::*;
pub use definition::*;

use std::{fs, io};
use std::io::{ErrorKind};
use std::path::PathBuf;


#[derive(Debug)]
pub(crate) struct LocalFile {
  path: PathBuf
}

impl LocalFile {
  pub(crate) fn get_filename<T: AsFilename>(
    base_path: &str, sub_directory: &str, filename: &T
  ) -> io::Result<Filename> {
    Ok(Filename(Self::get_name_inner(&mut PathBuf::from(base_path), sub_directory, filename)?))
  }

  pub(crate) fn convert_name(&mut self, convert_name: impl Fn(Option<String>) -> String) -> io::Result<()> {
    let prev = self.path.clone();

    let filename = self.path.file_name().ok_or(
      io::Error::new(ErrorKind::InvalidData, "Filename could not be read")
    )?.to_str().and_then(|x| Some(x.to_string()));
    self.path.pop();
    self.path.push(convert_name(filename));
    
    fs::rename(prev, &self.path)
  }

  fn get_name_inner<T: AsFilename>(
    path: &mut PathBuf, sub_directory: &str, filename: &T
  ) -> io::Result<String> {
    path.push(sub_directory);

    filename.as_filename(&path)
  }

  pub(crate) fn get_file<T: AsFilename>(
    base_path: &str, sub_directory: &str, filename: &T
  ) -> io::Result<LocalFile> {
    let mut pb = PathBuf::from(base_path);
    let name = Self::get_name_inner(&mut pb, sub_directory, filename)?;
    pb.push(&name);
    
    Ok(LocalFile {
      path: pb
    })
  }

  pub fn get_contents(&self) -> io::Result<String> {
    let reads = fs::read(&self.path)?;
    match String::from_utf8(reads.clone()) {
      Ok(o) => Ok(o),
      Err(_) => {
        // utf8 didn't work. string is probably in ANSI
        
        if let Some(encoding) = codepage::to_encoding(850) {
          let (str, _, ok) = encoding.decode(reads.as_slice());
          if ok { return Ok(str.to_string()) }
        }

        Err(io::Error::new(ErrorKind::Unsupported, "File was not encoded in utf8 or ANSI"))
      }
    }
  }

  pub fn write_contents(&self, contents: &String) -> bool {
    fs::write(&self.path, contents).is_ok()
  }
}
