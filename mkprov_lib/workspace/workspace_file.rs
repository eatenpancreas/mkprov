use LoadFileError::IncorrectFileFormat;
use encoding_rs::{UTF_8, WINDOWS_1252};
use std::{fs, io, marker::PhantomData};
use thiserror::Error;

use crate::workspace::{Workspace, from_file::FromFile};

pub struct WorkspaceFile<'w, F: FromFile> {
    _marker: PhantomData<F>,
    workspace: &'w Workspace,
    path: &'w str,
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

impl<'w, F: FromFile> WorkspaceFile<'w, F> {
    #[inline]
    pub fn get(workspace: &'w Workspace, path: &'w str) -> Self {
        Self { workspace, path, _marker: Default::default() }
    }

    /// Loads the file from the mod directory.
    ///
    /// # Errors
    ///
    /// Returns a `LoadFileError::IoError` if there is an issue reading the file from the filesystem.
    /// Returns a `LoadFileError::IncorrectFileFormat` if the file is not in UTF-8 or ANSI format.
    ///
    /// # Note
    ///
    /// If the file does not exist in the mod directory, use `Self::pull_source` to load it from
    /// the source directory into the mod directory first.
    pub fn load(&self) -> Result<F, LoadFileError<F>> {
        let mod_path = self.workspace.location().join(self.path);
        self.read_bytes(fs::read(mod_path)?)
    }

    pub fn load_either(&self) -> Result<F, LoadFileError<F>> {
        let mod_path = self.workspace.location().join(self.path);
        if !mod_path.exists() {
            let source_path = self.workspace.game_location.join(self.path);
            self.read_bytes(fs::read(source_path)?)
        } else {
            self.read_bytes(fs::read(mod_path)?)
        }
    }

    /// Loads the file from the source directory and writes it to the mod directory.
    /// # Warning
    /// Will override the file in the mod directory.
    pub fn pull_source(&self) -> Result<F, LoadFileError<F>> {
        let source_path = self.workspace.game_location.join(self.path);
        let bytes = fs::read(source_path)?;

        let mod_path = self.workspace.location().join(self.path);
        if let Some(parent) = mod_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(mod_path, &bytes)?;

        self.read_bytes(bytes)
    }

    pub fn save(&self, data: F) -> Result<(), SaveFileError<F>> {
        Ok(fs::write(
            self.workspace.location().join(self.path),
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

    pub fn in_mod(&self) -> bool {
        let mod_path = self.workspace.location().join(self.path);
        mod_path.exists()
    }

    pub fn in_source(&self) -> bool {
        let mod_path = self.workspace.game_location.join(self.path);
        mod_path.exists()
    }
}
