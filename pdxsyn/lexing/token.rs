use std::fmt::Display;

use crate::data::Literal;
use Token::*;

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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExplicitString(str) => write!(f, "\"{str}\""),
            Literal(Literal::Date(d)) => {
                write!(f, "{:04}.{:02}.{:02}", d.year(), d.month(), d.day())
            }
            Literal(Literal::F32(float, p)) => write!(f, "{float:.pre$}", pre = **p),
            Literal(Literal::I64(int)) => int.fmt(f),
            Literal(Literal::String(str)) => str.fmt(f),
            Comment(str) => write!(f, "#{str}"),
            Whitespace(str) => str.fmt(f),
            Equals => '='.fmt(f),
            BracketL => '{'.fmt(f),
            BracketR => '}'.fmt(f),
        }
    }
}
