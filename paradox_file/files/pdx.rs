use std::io;
use thiserror::Error;
use crate::{
    AsFilename, Config, LexerError, LocalFile, 
    Object, Parser, ParserError, RequireError
};

pub struct PdxFile {
    pub contents: Object,
    file: LocalFile,
}

#[derive(Error, Debug)]
pub enum PdxFileError {
    #[error("Something went wrong with lexing")]
    LexerError(Vec<LexerError>),
    #[error("Something went wrong with parsing")]
    ParserError(#[from] ParserError),
    #[error("File is not present in mod or base-game")]
    FileNotPresent,
    #[error("Something went wrong with getting files")]
    IoError(#[from] io::Error),
    #[error(transparent)]
    RequireError(#[from] RequireError)
}

impl PdxFile {
    pub fn save(&self) {
        if !self.file.write_contents(&self.contents.to_string()) {
            println!("Failed to save {:?}", self.file.path)
        }
    }
    
    pub fn rename_prov_name(&mut self, id: u16, to: String) -> io::Result<()> {
        self.file.convert_name(|_| {
            format!("{} - {}.txt", id, to)
        })
    }

    /// inspects mod directory, else if that doesn't exist, inspects game directory file
    pub fn inspect<T: AsFilename>(
        cfg: &Config, sub_directory: &str, filename: &T
    ) -> Result<Object, PdxFileError> {
        let file_contents;
        
        if let Ok(mod_file) = LocalFile::get_file(
            cfg.require_mod_directory()?, sub_directory, filename
        ) { file_contents = mod_file.get_contents()?; }
        else if let Ok(game_file) = LocalFile::get_file(
            cfg.require_game_directory()?, sub_directory, filename
        ) { file_contents = game_file.get_contents()?; } 
        else {
            return Err(PdxFileError::FileNotPresent)
        }

        let mut p = Parser::include_lexer(file_contents.as_str())
          .map_err(|lx_err| {PdxFileError::LexerError(lx_err)})?;
        Ok(p.parse()?)
    }

    /// Uses mod directory, else copies from game directory into mod directory and uses it
    pub fn pull<T: AsFilename>(
        cfg: &Config, sub_directory: &str, filename: &T
    ) -> Result<Self, PdxFileError> {
        let file_contents;
        let file;
        if let Ok(mod_file) = LocalFile::get_file(
            cfg.require_mod_directory()?, sub_directory, filename
        ) {
            file_contents = mod_file.get_contents()?;
            file = mod_file;
        } else if let Ok(filename) = LocalFile::get_filename(
            cfg.require_game_directory()?, sub_directory, filename
        ) {
            let game_file = LocalFile::get_file(
                cfg.require_game_directory()?, sub_directory, &filename)?;
            file_contents = game_file.get_contents()?;
            let mod_file = LocalFile::get_file(
                cfg.require_mod_directory()?, sub_directory, &filename)?;
            mod_file.write_contents(&file_contents);
            file = mod_file;
        } else {
            return Err(PdxFileError::FileNotPresent)
        }

        let mut p = Parser::include_lexer(file_contents.as_str())
          .map_err(|lx_err| {PdxFileError::LexerError(lx_err)})?;
        
        Ok(PdxFile { contents: p.parse()?, file })
          .map_err(|lx_err| {PdxFileError::LexerError(lx_err)})
    }
}
