use LoadFileError::IncorrectFileFormat;
use cli_prompts::{DisplayPrompt, prompts::Confirmation, style::ConfirmationStyle};
use encoding_rs::{UTF_8, WINDOWS_1252};
use std::{fs, io, marker::PhantomData, path::PathBuf, string::FromUtf8Error};
use text_diff::print_diff;
use thiserror::Error;

use crate::workspace::{Workspace, from_file::FromFile};

#[derive(PartialEq, Clone, Debug)]
pub struct WorkspaceFile<F> {
    _marker: PhantomData<F>,
    path: PathBuf,
    encoding: Option<Encoding>,
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
    #[error(transparent)]
    Utf8Error(#[from] FromUtf8Error),
    #[error("Error converting into file: {0}")]
    IntoFileError(F::IntoFileError),
    #[error("Workspace file has not set encoding")]
    NoEncodingSetForSaving,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Encoding {
    ANSI,
    UTF8,
}

impl<T> WorkspaceFile<T> {
    #[inline]
    pub fn set_encoding(&mut self, encoding: Encoding) { self.encoding = Some(encoding) }

    #[inline]
    pub fn in_mod(&self, workspace: &Workspace) -> bool {
        workspace.location().join(&self.path).exists()
    }

    #[inline]
    pub fn in_source(&self, workspace: &Workspace) -> bool {
        workspace
            .game_location
            .as_ref()
            .unwrap()
            .join(&self.path)
            .exists()
    }

    pub fn path(&self) -> &PathBuf { &self.path }

    #[inline]
    pub(crate) fn get(path: PathBuf) -> Self {
        Self { path: path.into(), _marker: Default::default(), encoding: None }
    }
}

impl WorkspaceFile<()> {
    #[inline]
    pub fn into_typed<F: FromFile>(self) -> WorkspaceFile<F> { WorkspaceFile::get(self.path) }
}

impl<F: FromFile> WorkspaceFile<F> {
    /// Loads the file from the mod directory.
    pub fn load_mod(&mut self, workspace: &Workspace) -> Result<F, LoadFileError<F>> {
        let mod_path = workspace.location().join(&self.path);
        self.read_bytes(fs::read(mod_path)?)
    }

    pub fn load_either(&mut self, workspace: &Workspace) -> Result<F, LoadFileError<F>> {
        let mod_path = workspace.location().join(&self.path);
        if !mod_path.exists() {
            let source_path = workspace.game_location.as_ref().unwrap().join(&self.path);
            self.read_bytes(fs::read(source_path)?)
        } else {
            self.read_bytes(fs::read(mod_path)?)
        }
    }

    pub fn save(
        &self,
        data: F,
        workspace: &Workspace,
        confirm: bool,
    ) -> Result<(), SaveFileError<F>> {
        let path = workspace.location().join(&self.path);

        let utf8 = data
            .into_file()
            .map_err(|e| SaveFileError::IntoFileError(e))?;

        let content = match self.encoding {
            Some(Encoding::ANSI) => &*WINDOWS_1252.encode(&utf8).0,
            Some(Encoding::UTF8) => utf8.as_bytes(),
            None => return Err(SaveFileError::NoEncodingSetForSaving),
        };

        if confirm {
            let original = WorkspaceFile::<String>::get(self.path.clone())
                .load_either(workspace)
                .unwrap();

            print_diff(&original, &utf8, " ");

            let conf = Confirmation::new(format!("Saving to {:?}, is that ok?", self.path))
                .style(ConfirmationStyle::default())
                .display()
                .unwrap();
            if !conf {
                return Ok(());
            }
        } else {
            println!("Saving to {:?}", self.path);
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        Ok(fs::write(path, content)?)
    }

    fn read_bytes(&mut self, bytes: Vec<u8>) -> Result<F, LoadFileError<F>> {
        if let (text, _, false) = UTF_8.decode(&bytes) {
            self.set_encoding(Encoding::UTF8);
            F::from_file(text).map_err(|e| LoadFileError::FromFileError(e))
        } else if let (text, _, false) = WINDOWS_1252.decode(&bytes) {
            self.set_encoding(Encoding::ANSI);
            F::from_file(text).map_err(|e| LoadFileError::FromFileError(e))
        } else {
            Err(IncorrectFileFormat)
        }
    }
}
