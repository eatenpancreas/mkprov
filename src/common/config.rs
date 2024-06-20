use std::env::current_exe;
use std::fs;
use std::path::{PathBuf};
use std::process::exit;
use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Debug, Args, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    #[arg(short = 'u', long)]
    #[serde(rename = "mod-url")]
    mod_url: Option<String>,
}

impl Config {
    pub fn current() -> Config {
        let pbuf = current_config_file();
        let path = pbuf.as_path();
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
        let Config { mod_url } = other;

        if let Some(mod_url) = mod_url { self.mod_url = Some(mod_url) }
    }

    pub fn override_all_fields(&mut self, other: Config) {
        *self = other
    }

    pub fn echo_all_fields(self) {
        let Config { mod_url } = self;
        println!("[mod-url]: {mod_url}");
    }

    pub fn require_mod_url(&self) -> &String {
        Self::require(&self.mod_url, "mod-url")
    }

    fn require<'a, T>(field: &'a Option<T>, name: &str) -> &'a T {
        if field.is_none() {
            eprintln!("Config field {name} is required for this command to work.");
            exit(1);
        }

        return field.as_ref().unwrap()
    }

    pub fn save(&self) {
        let pbuf = current_config_file();
        let path = pbuf.as_path();
        fs::write(path, toml::to_string(self).unwrap()).unwrap();
    }
}

fn current_config_file() -> PathBuf {
    let mut pbuf = current_exe().ok().unwrap();
    pbuf.pop();
    pbuf.push("mkprov_config.toml");
    pbuf
}