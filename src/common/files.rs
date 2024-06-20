use std::{fs, io};
use std::fs::DirEntry;
use std::path::PathBuf;

// pub fn dir_has_id(dir: &PathBuf, id: u16) -> bool {
//     find_id(dir, id).is_some()
// }

pub fn dir_get_id_filename(dir: &PathBuf, id: u16) -> Option<String> {
    if let Some(Ok(entry)) = find_id(dir, id) {
        let os = entry.file_name();
        os.to_str().and_then(|x| Some(x.to_string()))
    } else {
        None
    }
}

fn find_id(dir: &PathBuf, id: u16) -> Option<io::Result<DirEntry>> {
    let read = fs::read_dir(dir);
    if read.is_err() { return None; }
    let mut read = read.unwrap();
    let id = format!("{id} -");

    read.find(|dir_entry| {
        if let Some(dir_entry) = dir_entry.as_ref().ok()
            .and_then(|de| Some(de.file_name()))
        {
            dir_entry.to_str().and_then(|dir_entry| Some(
                dir_entry.starts_with(id.as_str())
            )).unwrap_or(false)
        } else {
            false
        }
    })
}