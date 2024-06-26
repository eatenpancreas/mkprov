use crate::common::dir_get_id_filename;
use std::fs;
use std::path::PathBuf;
use paradox_file::{Object, Parser};

pub struct Province {
    contents: Object,
    location: String,
}

impl Province {
    pub fn save(&self) {
        fs::write(&self.location, self.contents.to_string()).unwrap();
    }

    pub fn pull(id: u16, mut mod_dir: PathBuf, mut game_dir: PathBuf) -> Option<Self> {
        mod_dir.push("history/provinces/");
        game_dir.push("history/provinces/");
        let file_contents;
        let location;

        if let Some(filename) = dir_get_id_filename(&mod_dir, id) {
            println!("Getting file from mod dir: {filename}");

            mod_dir.push(filename);
            let mod_path = mod_dir.to_str()?;
            location = mod_path.to_string();

            file_contents = String::from_utf8(fs::read(mod_path).ok()?).ok()?;
        } else if let Some(filename) = dir_get_id_filename(&game_dir, id) {
            println!("Getting file from base game dir: {filename}");

            game_dir.push(&filename);
            let game_path = game_dir.to_str()?;

            mod_dir.push(filename);
            let mod_path = mod_dir.to_str()?;
            location = mod_path.to_string();

            file_contents = String::from_utf8(fs::read(&game_path).ok()?).ok()?;

            fs::write(mod_path, &file_contents).ok()?;
        } else {
            eprintln!("province ID {id} is not present in mod or basegame");
            return None;
        }
        
        let mut p = Parser::include_lexer(file_contents.as_str()).unwrap();
        Some(Province {
            contents: p.parse().unwrap(),
            location,
        })
    }
}
