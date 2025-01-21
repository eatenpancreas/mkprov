mod from_file;
mod workspace_file;

pub use from_file::*;

use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use derived_deref::{Deref, DerefMut};
use pdxsyn::{Document, syntax::RootObject};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use workspace_file::WorkspaceFile;

pub trait ConfigTrait: Default + Serialize + DeserializeOwned {}
impl<T: Default + Serialize + DeserializeOwned> ConfigTrait for T {}

#[derive(Default, Serialize, Deserialize)]
pub enum Game {
    #[default]
    Eu4,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub game_location: PathBuf,
    pub game: Game,
    pub line_wrap: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self { game: Default::default(), game_location: Default::default(), line_wrap: Some(4) }
    }
}

#[derive(Deref, DerefMut)]
pub struct Workspace {
    location: PathBuf,
    #[target]
    config: Config,
}

impl Workspace {
    #[inline]
    pub fn load() -> io::Result<Option<Self>> { Self::custom_load(std::env::current_dir()?) }
    #[inline]
    pub fn create() -> io::Result<Self> { Self::custom_create(std::env::current_dir()?) }

    #[inline]
    pub fn get_any_file<'a, F: FromFile>(&'a self, path: &'a str) -> WorkspaceFile<'a, F> {
        WorkspaceFile::get(self, path)
    }

    #[inline]
    pub fn get_string_file<'a>(&'a self, path: &'a str) -> WorkspaceFile<'a, String> {
        WorkspaceFile::get(self, path)
    }

    #[inline]
    pub fn get_csv_file<'a, T: Serialize + DeserializeOwned>(
        &'a self,
        path: &'a str,
    ) -> WorkspaceFile<'a, Csv<T>> {
        WorkspaceFile::get(self, path)
    }

    #[inline]
    pub fn get_pdx_file<'a>(&'a self, path: &'a str) -> WorkspaceFile<'a, (Document, RootObject)> {
        WorkspaceFile::get(self, path)
    }

    #[inline]
    pub fn get_pdx_file_unparsed<'a>(&'a self, path: &'a str) -> WorkspaceFile<'a, Document> {
        WorkspaceFile::get(self, path)
    }

    #[inline]
    pub fn location(&self) -> &PathBuf { &self.location }

    pub fn custom_load(mut current_path: PathBuf) -> io::Result<Option<Self>> {
        while let true = current_path.pop() {
            let config_path = current_path.join(".mkprov");
            if config_path.exists() {
                let bytes = fs::read(&config_path)?;
                let config: Config = bincode::deserialize(&bytes)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                return Ok(Some(Self { location: current_path, config }));
            }
        }

        Ok(None)
    }

    pub fn custom_create(current_path: PathBuf) -> io::Result<Self> {
        let config_path = current_path.join(".mkprov");

        let config = Config::default();
        let mut file = fs::File::create(&config_path)?;
        let encoded = bincode::serialize(&config).unwrap();
        file.write_all(&encoded)?;

        Ok(Self { config, location: current_path })
    }
}
