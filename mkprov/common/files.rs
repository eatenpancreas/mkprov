use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};
use paradox_file::{Object, Parser};
use crate::common::{Config};

pub struct PdxFile {
    pub(crate) contents: Object,
    location: String,
}

pub trait AsFilename {
    fn as_filename(&self, dir: &PathBuf) -> Option<String>;
}

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

impl PdxFile {
    pub fn save(&self) {
        fs::write(&self.location, self.contents.to_string()).unwrap();
    }

    pub fn pull<T: AsFilename>(file_n: T, cfg: Config, sub_dir: &str) -> Option<Self> {
        let mut mod_dir = PathBuf::from(cfg.require_mod_directory());
        let mut game_dir = PathBuf::from(cfg.require_game_directory());
        mod_dir.push(sub_dir);
        game_dir.push(sub_dir);
        let file_contents;
        let location;

        if let Some(filename) = file_n.into_filename(&mod_dir, id) {
            println!("Getting file from mod dir: {filename}");

            mod_dir.push(filename);
            let mod_path = mod_dir.to_str()?;
            location = mod_path.to_string();

            file_contents = String::from_utf8(fs::read(mod_path).ok()?).ok()?;
        } else if let Some(filename) = file_n.into_filename(&game_dir, id) {
            println!("Getting file from base game dir: {filename}");

            game_dir.push(&filename);
            let game_path = game_dir.to_str()?;

            mod_dir.push(filename);
            let mod_path = mod_dir.to_str()?;
            location = mod_path.to_string();

            file_contents = String::from_utf8(fs::read(&game_path).ok()?).ok()?;

            fs::write(mod_path, &file_contents).ok()?;
        } else {
            eprintln!("File {file_n} is not present in mod or basegame");
            return None;
        }

        let mut p = Parser::include_lexer(file_contents.as_str()).unwrap();
        Some(PdxFile {
            contents: p.parse().unwrap(),
            location,
        })
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
        if let Some(dir_entry) = dir_entry.as_ref().ok().and_then(|de| Some(de.file_name())) {
            dir_entry
                .to_str()
                .and_then(|dir_entry| Some(dir_entry.starts_with(id.as_str())))
                .unwrap_or(false)
        } else {
            false
        }
    })
}
