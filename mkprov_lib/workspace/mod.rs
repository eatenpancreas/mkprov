mod combined_folder;
mod from_file;
mod workspace_file;

pub use from_file::*;
pub use workspace_file::*;

use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use derived_deref::{Deref, DerefMut};
use pdxsyn::{Document, syntax::RootObject};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub trait ConfigTrait: Default + Serialize + DeserializeOwned {}
impl<T: Default + Serialize + DeserializeOwned> ConfigTrait for T {}

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Game {
    #[default]
    Eu4,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub game_location: Option<PathBuf>,
    pub game: Option<Game>,
    pub line_wrap: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self { game: Default::default(), game_location: Default::default(), line_wrap: Some(4) }
    }
}

#[derive(Deref, DerefMut, Clone, Debug)]
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
    pub fn game_location_exists(&self) -> bool {
        self.game_location.as_ref().is_some_and(|gl| gl.exists())
    }

    #[inline]
    pub fn get_any_file(&self, path: impl Into<PathBuf>) -> WorkspaceFile<()> {
        WorkspaceFile::get(strip_workspace(path.into(), self))
    }

    #[inline]
    pub fn get_string_file(&self, path: impl Into<PathBuf>) -> WorkspaceFile<String> {
        WorkspaceFile::get(strip_workspace(path.into(), self))
    }

    #[inline]
    pub fn get_csv_file<T: Serialize + DeserializeOwned>(
        &self,
        path: impl Into<PathBuf>,
    ) -> WorkspaceFile<Csv<T>> {
        WorkspaceFile::get(strip_workspace(path.into(), self))
    }

    #[inline]
    pub fn get_any_csv_file(&self, path: impl Into<PathBuf>) -> WorkspaceFile<AnyCsv> {
        WorkspaceFile::get(strip_workspace(path.into(), self))
    }

    #[inline]
    pub fn get_any_yaml_file(&self, path: impl Into<PathBuf>) -> WorkspaceFile<AnyYaml> {
        WorkspaceFile::get(strip_workspace(path.into(), self))
    }

    #[inline]
    pub fn get_pdx_file(&self, path: impl Into<PathBuf>) -> WorkspaceFile<(Document, RootObject)> {
        WorkspaceFile::get(strip_workspace(path.into(), self))
    }

    #[inline]
    pub fn get_pdx_file_unparsed(&self, path: impl Into<PathBuf>) -> WorkspaceFile<Document> {
        WorkspaceFile::get(strip_workspace(path.into(), self))
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

fn strip_workspace(path: PathBuf, wk: &Workspace) -> PathBuf {
    if let Ok(t) = path.strip_prefix(&wk.location) {
        return t.into();
    }
    if let Some(source) = &wk.game_location {
        if let Ok(t) = path.strip_prefix(source) {
            return t.into();
        }
    }

    path
}
