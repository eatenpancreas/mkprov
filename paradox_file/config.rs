
use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::{fs, io};
use std::path::PathBuf;
use std::process::exit;
use crate::if_err;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "mod-directory")]
    mod_directory: Option<String>,
    #[serde(rename = "game-directory")]
    game_directory: Option<String>,
}

pub struct RequireError(String);

impl Config {
    pub fn current() -> Config {
        let p_buf = current_config_file();
        let path = p_buf.as_path();
        if let Ok(true) = path.try_exists() {
            let file = String::from_utf8(fs::read(path).unwrap()).unwrap();
            toml::from_str(file.as_str()).unwrap()
        } else {
            let cfg = Config::default();
            fs::write(path, toml::to_string(&cfg).unwrap()).unwrap();
            cfg
        }
    }

    pub fn set_fields(&mut self, other: Config) {
        let Config {
            mod_directory,
            game_directory,
        } = other;

        if let Some(cfg) = mod_directory {
            self.mod_directory = Some(cfg)
        }
        if let Some(cfg) = game_directory {
            self.game_directory = Some(cfg)
        }
    }

    pub fn override_all_fields(&mut self, other: Config) {
        *self = other
    }

    pub fn echo_all_fields(self) {
        let Config {
            mod_directory,
            game_directory,
        } = self;
        println!("[mod-directory]: {mod_directory:?}");
        println!("[game-directory]: {game_directory:?}");
    }

    pub fn require_mod_directory(&self) -> Result<&String, RequireError> {
        Self::require(&self.mod_directory, "mod-directory")
    }

    pub fn require_game_directory(&self) -> Result<&String, RequireError> {
        Self::require(&self.game_directory, "game-directory")
    }

    fn require<'a, T>(field: &'a Option<T>, name: &str) -> Result<&'a T, RequireError> {
        field.as_ref().ok_or(RequireError(format!(
            "Config field {name} is required for this command to work. \
            (try running mkprov cfg set --{name} my_value)"
        )))
    }

    pub fn save(&self) -> bool {
        let p_buf = if_err!(current_config_file());
        let path = p_buf.as_path();
        fs::write(path, if_err!(toml::to_string(self))).is_ok()
    }
}

fn current_config_file() -> io::Result<PathBuf> {
    let mut p_buf = current_exe()?;
    p_buf.pop();
    p_buf.push("mkprov_config.toml");
    p_buf
}
