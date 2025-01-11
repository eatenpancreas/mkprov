use crate::data::Literal;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    ExplicitString(String),
    Literal(Literal),
    Comment(String),
    Whitespace(String),
    Equals,
    BracketL,
    BracketR,
}
