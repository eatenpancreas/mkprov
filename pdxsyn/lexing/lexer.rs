use crate::ParseDateError;

pub use super::token::*;

use thiserror::Error;

/// A lexer for paradox syntax, implemented as an iterator that returns syntax tokens.
///
/// Some examples:
/// ```
/// use pdxsyn::*;
/// let lexer_output = Lexer::new("-0.110").next().unwrap().unwrap();
/// assert_eq!(
///    lexer_output,
///    Token::Literal(Literal::F32(-0.11, Precision::new(3)))
///);
/// ```
#[derive(Clone)]
pub struct Lexer {
    cursor: usize,
    characters: Vec<char>,
}

#[derive(Error, Debug, Clone)]
pub enum LexerError {
    #[error("Unexpected end of file at pos {0})")]
    UnexpectedEndOfFile(usize),
    #[error("Unexpected end of line at pos {0}")]
    UnexpectedEndOfLine(usize),
    #[error("Too many .'s in number: {0} at pos {1}")]
    TooManyDots(usize, usize),
    #[error("Unexpected '{0}' at pos {1}")]
    UnexpectedToken(char, usize),
    #[error("Parsing Date error: {0} at pos {1}")]
    DateError(ParseDateError, usize),
}

impl LexerError {
    pub(crate) fn err(self) -> Option<Result<Token, Self>> { Some(Err(self)) }
}

impl Lexer {
    /// Creates a new lexer.
    pub fn new(string: &str) -> Lexer { Lexer { cursor: 0, characters: string.chars().collect() } }

    /// Returns the next character (if available) and advances the cursor.
    pub(crate) fn pop(&mut self) -> Option<char> {
        let item = self.peek();
        self.advance();
        item
    }

    pub(crate) fn peek(&self) -> Option<char> { self.characters.get(self.cursor).map(|c| *c) }
    pub(crate) fn advance(&mut self) { self.cursor += 1; }
    pub(crate) fn cursor(&mut self) -> usize { self.cursor - 1 }
}
