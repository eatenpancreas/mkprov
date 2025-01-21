mod data;
mod lexing;
mod syntax_editing;

use parsing::ParseDocumentError;
use thiserror::Error;
pub use {data::*, lexing::*, syntax_editing::*};

#[derive(Error, Debug)]
pub enum PdxError {
    #[error(transparent)]
    Lex(#[from] LexerError),
    #[error(transparent)]
    Parse(#[from] ParseDocumentError),
}
