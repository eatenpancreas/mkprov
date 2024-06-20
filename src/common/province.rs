use std::fs;
use std::path::PathBuf;
use crate::common::dir_get_id_filename;

pub struct Province {
    file_contents: String,
    location: String
}

impl Province {
    pub fn save(&self) {
        fs::write(&self.location, &self.file_contents).unwrap();
    }

    fn set_field(&mut self, field: &str, set_to: String) {
        let mut new_lines = vec![];
        let mut lines = self.file_contents.split('\n');
        let mut found_field = false;

        while let Some(line) = lines.next() {
            let mut sides = line.split('=');

            let lhs = sides.next().unwrap_or("").split_whitespace().next().unwrap_or("");
            let rhs = sides.next().unwrap_or("").split_whitespace().next().unwrap_or("");

            if sides.next().is_some() || lhs == "" || rhs == "" {
                new_lines.push(line);
            } else if rhs == "{" {
                new_lines.push(line);
                while let Some(line) = lines.next() {
                    new_lines.push(line);
                    if line.contains('{') {
                        break;
                    }
                }
            } else if !found_field && lhs == field {
                new_lines.push(format!("{field} = {set_to}").as_str());
                found_field = true;
            } else {
                new_lines.push(line);
            }
        }

        let mut lines = String::new();
        new_lines.iter().for_each(|line| {
            lines.push_str(line);
            lines.push('\n')
        });


        println!("{lines}");
        self.file_contents = lines;
    }

    pub fn set_owner(&mut self, tag: String) {
        self.set_field("owner",  tag);
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

        Some(Province {
            file_contents,
            location
        })
    }
}