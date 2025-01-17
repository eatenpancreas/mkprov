mod lexer;
mod parsed_structure;
mod parser;
mod token;

pub use {lexer::*, parsed_structure::*, parser::*, token::*};
pub mod file;

#[derive(Debug, PartialEq)]
pub enum Literal {
    U8(u8),
    U16(u16),
    F32(f32),
    String(String),
    Date(Date),
}

impl Literal {
    pub fn parse(content: String) -> Option<Literal> {
        if content.starts_with(|ch: char| ch.is_numeric()) {
            parse_numeral(&content)
        } else {
            // regular string
            Some(Literal::String(content))
        }
    }
}

fn parse_numeral(content: &str) -> Option<Literal> {
    let split: Vec<&str> = content.split('.').collect();
    if split.len() == 3 {
        Some(
            Date::new(
                split[0].parse().ok()?,
                split[1].parse().ok()?,
                split[2].parse().ok()?,
            )
            .into_literal(),
        )
    } else if split.len() == 2 {
        Some(Literal::F32(content.parse().ok()?))
    } else if split.len() == 1 {
        if let Ok(u8) = split[0].parse() {
            Some(Literal::U8(u8))
        } else {
            Some(Literal::U16(split[0].parse().ok()?))
        }
    } else {
        None
    }
}
