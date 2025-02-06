use std::fmt::Display;

use crate::data::Literal;
use Token::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Literal(Literal),
    Comment(String),
    Whitespace(String),
    Equals,
    BracketL,
    BracketR,
}

impl Token {
    pub fn name(&self) -> &'static str {
        match self {
            Token::Whitespace(_) => "Whitespace",
            Token::Literal(_) => "Literal",
            Token::Comment(_) => "Comment",
            Token::Equals => "Equals",
            Token::BracketR => "Bracket right",
            Token::BracketL => "Bracket left",
        }
    }

    pub fn into_literal(self) -> Option<Literal> {
        match self {
            Literal(lit) => Some(lit),
            _ => None,
        }
    }

    pub fn as_literal(&self) -> Option<&Literal> {
        match self {
            Literal(lit) => Some(lit),
            _ => None,
        }
    }

    pub fn as_literal_mut(&mut self) -> Option<&mut Literal> {
        match self {
            Literal(lit) => Some(lit),
            _ => None,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Literal(_) => true,
            _ => false,
        }
    }

    pub fn is_bracket_l(&self) -> bool {
        match self {
            BracketL => true,
            _ => false,
        }
    }

    pub fn is_bracket_r(&self) -> bool {
        match self {
            BracketR => true,
            _ => false,
        }
    }

    pub fn is_equals(&self) -> bool {
        match self {
            Equals => true,
            _ => false,
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match self {
            Whitespace(_) => true,
            _ => false,
        }
    }

    pub fn is_delimiter(&self) -> bool {
        match self {
            Comment(_) => true,
            Whitespace(_) => true,
            _ => false,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal(Literal::ExplicitString(str)) => write!(f, "\"{str}\""),
            Literal(Literal::Date(d)) => d.fmt(f),
            Literal(Literal::F32(float, p)) => {
                write!(f, "{float:.pre$}", pre = **p)
            }
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
