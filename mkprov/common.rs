use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};
use paradox_file::{AsFilename};

#[derive(Clone, Copy, Debug)]
pub struct Id(pub u16);

impl AsFilename for Id {
    fn as_filename(&self, dir: &PathBuf) -> io::Result<String> {
        let entry = find_id(dir, self.0)?;
        let os = entry.file_name();
        match os.to_str() {
            Some(osstr) => Ok(osstr.to_string()),
            None => Err(io::Error::new(io::ErrorKind::Unsupported,
                                       "Could not parse filename"))
        }
    }
}

fn find_id(dir: &PathBuf, id: u16) -> io::Result<DirEntry> {
    let mut read = fs::read_dir(dir)?;
    let id = format!("{id} -");

    read.find(|dir_entry| {
        if let Some(dir_entry) = dir_entry.as_ref().ok()
            .and_then(|de| Some(de.file_name())) {
            dir_entry
                .to_str()
                .and_then(|dir_entry| Some(dir_entry.starts_with(id.as_str())))
                .unwrap_or(false)
        } else {
            false
        }
    }).unwrap_or(Err(io::Error::new(io::ErrorKind::NotFound, "Could not find Id")))
}
