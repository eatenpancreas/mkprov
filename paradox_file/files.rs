use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use crate::{Config, Object, Parser};

pub trait AsFilename: Debug {
  fn as_filename(&self, dir: &PathBuf) -> Option<String>;
}

impl AsFilename for &str {
  fn as_filename(&self, dir: &PathBuf) -> Option<String> {
    Some(format!("{}/{self}", dir.to_str()?))
  }
}

pub struct PdxFile {
  pub contents: Object,
  location: String,
}

impl PdxFile {
  pub fn save(&self) {
    fs::write(&self.location, self.contents.to_string()).unwrap();
  }

  /// inspects mod directory, else if that doesn't exist, inspects game directory file
  pub fn inspect<T: AsFilename>(file_n: T, cfg: &Config, sub_dir: &str) -> Option<Object> {
    let mut mod_dir = PathBuf::from(cfg.require_mod_directory());
    let mut game_dir = PathBuf::from(cfg.require_game_directory());
    mod_dir.push(sub_dir);
    game_dir.push(sub_dir);
    let file_contents;

    if let Some(filename) = file_n.as_filename(&mod_dir) {
      println!("Getting file from mod dir: {filename}");

      mod_dir.push(filename);
      let mod_path = mod_dir.to_str()?;

      file_contents = String::from_utf8(fs::read(mod_path).ok()?).ok()?;
    } else if let Some(filename) = file_n.as_filename(&game_dir) {
      println!("Getting file from base game dir: {filename}");

      game_dir.push(&filename);
      let game_path = game_dir.to_str()?;

      file_contents = String::from_utf8(fs::read(&game_path).ok()?).ok()?;
    } else {
      eprintln!("File {file_n:?} is not present in mod or basegame");
      return None;
    }

    let mut p = Parser::include_lexer(file_contents.as_str()).unwrap();
    Some(p.parse().unwrap())
  }

  /// Uses mod directory, else copies from game directory into mod directory and uses it
  pub fn pull<T: AsFilename>(file_n: T, cfg: &Config, sub_dir: &str) -> Option<Self> {
    let mut mod_dir = PathBuf::from(cfg.require_mod_directory());
    let mut game_dir = PathBuf::from(cfg.require_game_directory());
    mod_dir.push(sub_dir);
    game_dir.push(sub_dir);
    let file_contents;
    let location;

    if let Some(filename) = file_n.as_filename(&mod_dir) {
      println!("Getting file from mod dir: {filename}");

      mod_dir.push(filename);
      let mod_path = mod_dir.to_str()?;
      location = mod_path.to_string();

      file_contents = String::from_utf8(fs::read(mod_path).ok()?).ok()?;
    } else if let Some(filename) = file_n.as_filename(&game_dir) {
      println!("Getting file from base game dir: {filename}");

      game_dir.push(&filename);
      let game_path = game_dir.to_str()?;

      mod_dir.push(filename);
      let mod_path = mod_dir.to_str()?;
      location = mod_path.to_string();

      file_contents = String::from_utf8(fs::read(&game_path).ok()?).ok()?;

      fs::write(mod_path, &file_contents).ok()?;
    } else {
      eprintln!("File {file_n:?} is not present in mod or basegame");
      return None;
    }

    let mut p = Parser::include_lexer(file_contents.as_str()).unwrap();
    Some(PdxFile {
      contents: p.parse().unwrap(),
      location,
    })
  }
}
