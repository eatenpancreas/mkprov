use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};
use paradox_file::{AsFilename};

#[derive(Clone, Copy)]
pub struct Id(pub u16);

impl AsFilename for Id {
    fn as_filename(&self, dir: &PathBuf) -> Option<String> {
        if let Some(Ok(entry)) = find_id(dir, self.0) {
            let os = entry.file_name();
            os.to_str().and_then(|x| Some(x.to_string()))
        } else {
            None
        }
    }
}

fn find_id(dir: &PathBuf, id: u16) -> Option<io::Result<DirEntry>> {
    let read = fs::read_dir(dir);
    if read.is_err() {
        return None;
    }
    let mut read = read.unwrap();
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
    })
}
