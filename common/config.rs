use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::string::FromUtf8Error;
use std::{fs, io};
use thiserror::Error;

fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    mod_directory: Option<String>,
    game_directory: Option<String>,
    #[serde(default = "default_true")]
    pub is_first_time: bool,
}

#[derive(Error, Debug)]
pub struct RequireError(String);

impl Display for RequireError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File error occurred")]
    IoError(#[from] io::Error),
    #[error("Config was not in UTF8")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("TOML was not able to be deserialized")]
    TomlDeserializeError(#[from] toml::de::Error),
    #[error("TOML was not able to be serialized")]
    TomlSerializeError(#[from] toml::ser::Error),
}

impl Config {
    pub fn current() -> Result<Config, ConfigError> {
        let p_buf = current_config_file()?;
        let path = p_buf.as_path();
        if let Ok(true) = path.try_exists() {
            let file = String::from_utf8(fs::read(path)?)?;
            Ok(toml::from_str(file.as_str())?)
        } else {
            let cfg = Config::default();
            fs::write(path, toml::to_string(&cfg)?)?;
            Ok(cfg)
        }
    }

    pub fn echo_all_fields(&self) {
        let Config {
            mod_directory,
            game_directory,
            ..
        } = self;

        println!(
            "[mod-directory]: {}",
            Self::echo_str(mod_directory).unwrap_or("")
        );
        println!(
            "[game-directory]: {}",
            Self::echo_str(game_directory).unwrap_or("")
        );
    }

    fn echo_str(string: &Option<String>) -> Option<&str> {
        string.as_ref().and_then(|x| Some(x.as_str()))
    }

    pub fn require_mod_directory(&self) -> Result<&String, RequireError> {
        Self::require(&self.mod_directory, "mod-directory")
    }
    pub fn set_mod_directory(&mut self, to: Option<String>) {
        self.mod_directory = to;
    }

    pub fn require_game_directory(&self) -> Result<&String, RequireError> {
        Self::require(&self.game_directory, "game-directory")
    }
    pub fn set_game_directory(&mut self, to: Option<String>) {
        self.game_directory = to;
    }

    fn require<'a, T>(field: &'a Option<T>, name: &str) -> Result<&'a T, RequireError> {
        field.as_ref().ok_or(RequireError(format!(
            "Config field {name} is required for this command to work. \
            (try running mkprov cfg set --{name} my_value)"
        )))
    }

    pub fn save(&self) -> bool {
        let p_buf = if let Ok(pb) = current_config_file() {
            pb
        } else {
            return false;
        };
        let content = if let Ok(pb) = toml::to_string(self) {
            pb
        } else {
            return false;
        };

        fs::write(p_buf.as_path(), content).is_ok()
    }
}

fn current_config_file() -> io::Result<PathBuf> {
    let mut p_buf = current_exe()?;
    p_buf.pop();
    p_buf.push("mkprov_config.toml");
    Ok(p_buf)
}
