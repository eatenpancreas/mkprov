use std::fmt::{Display, Formatter};
use crate::{Literal, Location, parse_numeral};

#[derive(Debug, PartialEq)]
pub struct Token {
  pub(crate) location: Location,
  pub(crate) token_type: Option<TokenType>,
  /// true if token is declared with " around it, therefore always a string
  pub(crate) is_lit_stringed: bool
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
  Literal(Literal),
  BracketR,
  BracketL,
  Equals
}

impl Display for TokenType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      TokenType::Literal(lit) => lit.to_string(),
      TokenType::BracketR => "}".to_string(),
      TokenType::BracketL => "{".to_string(),
      TokenType::Equals => "=".to_string(),
    };
    write!(f, "{}", str)
  }
}

impl Token {
  pub fn new(location: usize, content: &str, is_lit_stringed: bool) -> Token {
    Token {
      location: Location(location),
      token_type: Self::get_type(content.to_string(), is_lit_stringed),
      is_lit_stringed,
    }
  }
  
  pub fn get_type(content: String, is_lit_stringed: bool) -> Option<TokenType> {
    if is_lit_stringed {
      Some(TokenType::Literal(Literal::String(content)))
    } else if content.starts_with(|ch: char| ch.is_numeric()) {
      parse_numeral(&content).and_then(|n| Some(TokenType::Literal(n)))
      // special characters
    } else if content == "=" {
      Some(TokenType::Equals)
    } else if content == "{" {
      Some(TokenType::BracketL)
    } else if content == "}" {
      Some(TokenType::BracketR)
    } else {
      // regular string
      Some(TokenType::Literal(Literal::String(content)))
    }
  }
  
  pub fn is_equals(&self) -> bool {
    if let Some(TokenType::Equals) = self.token_type
    { true } else { false }
  }

  pub fn is_literal(&self) -> bool {
    if let Some(TokenType::Literal(..)) = self.token_type
    { true } else { false }
  }

  pub fn is_bracket_r(&self) -> bool {
    if let Some(TokenType::BracketR) = self.token_type
    { true } else { false }
  }

  pub fn is_bracket_l(&self) -> bool {
    if let Some(TokenType::BracketL) = self.token_type
    { true } else { false }
  }
}