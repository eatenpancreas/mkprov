use std::process::exit;
use crate::{AsFilename, Config, LocalFile, Object, Parser};

pub struct PdxFile {
    pub contents: Object,
    file: LocalFile,
}

impl PdxFile {
    pub fn save(&self) {
        if !self.file.write_contents(&self.contents.to_string()) {
            println!("Failed to save {}", self.file.path)
        }
    }

    /// inspects mod directory, else if that doesn't exist, inspects game directory file
    pub fn inspect<T: AsFilename>(cfg: &Config, sub_directory: &str, filename: &T) -> Option<Object> {
        let file_contents;
        if let Some(mod_file) = LocalFile::get_file(cfg.require_mod_directory(), sub_directory, filename) {
            file_contents = mod_file.get_contents()?;
        } else if let Some(game_file) = LocalFile::get_file(cfg.require_game_directory(), sub_directory, filename) {
            file_contents = game_file.get_contents()?;
        } else {
            eprintln!("File {filename:?} is not present in mod or basegame");
            return None;
        }

        let mut p = Parser::include_lexer(file_contents.as_str()).unwrap();
        Some(p.parse().unwrap())
    }

    /// Uses mod directory, else copies from game directory into mod directory and uses it
    pub fn pull<T: AsFilename>(cfg: &Config, sub_directory: &str, filename: &T) -> Self {
        let file_contents;
        let file;
        if let Some(mod_file) = LocalFile::get_file(cfg.require_mod_directory(), sub_directory, filename) {
            file_contents = mod_file.get_contents().unwrap();
            file = mod_file;
        } else if let Some(filename) = LocalFile::get_filename(cfg.require_game_directory(), sub_directory, filename) {
            let game_file = LocalFile::get_file(cfg.require_game_directory(), sub_directory, &filename).unwrap();
            file_contents = game_file.get_contents().unwrap();
            let mod_file = LocalFile::get_file(cfg.require_mod_directory(), sub_directory, &filename).unwrap();
            mod_file.write_contents(&file_contents);
            file = mod_file;
        } else {
            eprintln!("File {filename:?} is not present in mod or basegame");
            exit(1)
        }

        let mut p = Parser::include_lexer(file_contents.as_str()).unwrap();
        PdxFile {
            contents: p.parse().unwrap(),
            file,
        }
    }
}
