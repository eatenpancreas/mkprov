use LoadFileError::IncorrectFileFormat;
use encoding_rs::{UTF_8, WINDOWS_1252};
use std::{fs, io, marker::PhantomData, path::PathBuf};
use thiserror::Error;

use crate::workspace::{Workspace, from_file::FromFile};

#[derive(PartialEq, Clone, Debug)]
pub struct WorkspaceFile<F> {
    _marker: PhantomData<F>,
    path: PathBuf,
}

#[derive(Error, Debug)]
pub enum LoadFileError<F: FromFile> {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("File was not in utf-8 or ANSI")]
    IncorrectFileFormat,
    #[error("Error converting from file: {0}")]
    FromFileError(F::FromFileError),
}

#[derive(Error, Debug)]
pub enum SaveFileError<F: FromFile> {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("Error converting into file: {0}")]
    IntoFileError(F::IntoFileError),
}

impl<T> WorkspaceFile<T> {
    pub fn in_mod(&self, workspace: &Workspace) -> bool {
        let mod_path = workspace.location().join(&self.path);
        mod_path.exists()
    }

    pub fn in_source(&self, workspace: &Workspace) -> bool {
        let mod_path = workspace.game_location.as_ref().unwrap().join(&self.path);
        mod_path.exists()
    }

    pub fn path(&self) -> &PathBuf { &self.path }

    #[inline]
    pub(crate) fn get(path: PathBuf) -> Self {
        Self { path: path.into(), _marker: Default::default() }
    }
}

impl WorkspaceFile<()> {
    #[inline]
    pub fn into_typed<F: FromFile>(self) -> WorkspaceFile<F> { WorkspaceFile::get(self.path) }
}

impl<F: FromFile> WorkspaceFile<F> {
    /// Loads the file from the mod directory.
    pub fn load_mod(&self, workspace: &Workspace) -> Result<F, LoadFileError<F>> {
        let mod_path = workspace.location().join(&self.path);
        self.read_bytes(fs::read(mod_path)?)
    }

    pub fn load_either(&self, workspace: &Workspace) -> Result<F, LoadFileError<F>> {
        let mod_path = workspace.location().join(&self.path);
        if !mod_path.exists() {
            let source_path = workspace.game_location.as_ref().unwrap().join(&self.path);
            self.read_bytes(fs::read(source_path)?)
        } else {
            self.read_bytes(fs::read(mod_path)?)
        }
    }

    pub fn save(&self, data: F, workspace: &Workspace) -> Result<(), SaveFileError<F>> {
        let path = workspace.location().join(&self.path);
        println!("saving to: {path:?}");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(fs::write(
            path,
            data.into_file()
                .map_err(|e| SaveFileError::IntoFileError(e))?,
        )?)
    }

    fn read_bytes(&self, bytes: Vec<u8>) -> Result<F, LoadFileError<F>> {
        if let (text, _, false) = UTF_8.decode(&bytes) {
            F::from_file(text).map_err(|e| LoadFileError::FromFileError(e))
        } else if let (text, _, false) = WINDOWS_1252.decode(&bytes) {
            F::from_file(text).map_err(|e| LoadFileError::FromFileError(e))
        } else {
            Err(IncorrectFileFormat)
        }
    }
}
