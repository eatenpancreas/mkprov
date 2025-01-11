use thiserror::Error;

use super::Date;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    U32(u32),
    F32(f32),
    String(String),
    Date(Date),
}

#[derive(Error, Debug)]
pub enum ParseNumericError {
    #[error("Invalid numeric value: {0}")]
    InvalidNumericType(String),
}

impl Literal {
    pub fn parse(content: String) -> Option<Self> {
        if content.starts_with(|ch: char| ch.is_numeric()) {
            Self::parse_numeric(&content)
        } else {
            // regular string
            Some(Self::String(content))
        }
    }

    pub fn parse_numeric(content: &String) -> Option<Self> {
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
            Some(Self::F32(content.parse().ok()?))
        } else if split.len() == 1 {
            Some(Self::U32(split[0].parse().ok()?))
        } else {
            None
        }
    }
}

pub trait IntoLiteral {
    fn into_literal(self) -> Literal;
}

impl IntoLiteral for u8 {
    fn into_literal(self) -> Literal {
        Literal::U32(self as u32)
    }
}

impl IntoLiteral for u16 {
    fn into_literal(self) -> Literal {
        Literal::U32(self as u32)
    }
}

impl IntoLiteral for u32 {
    fn into_literal(self) -> Literal {
        Literal::U32(self as u32)
    }
}

impl IntoLiteral for Date {
    fn into_literal(self) -> Literal {
        Literal::Date(self)
    }
}

impl IntoLiteral for &str {
    fn into_literal(self) -> Literal {
        Literal::String(self.to_string())
    }
}

impl IntoLiteral for String {
    fn into_literal(self) -> Literal {
        Literal::String(self)
    }
}

impl IntoLiteral for &String {
    fn into_literal(self) -> Literal {
        Literal::String(self.to_string())
    }
}

impl IntoLiteral for f32 {
    fn into_literal(self) -> Literal {
        Literal::F32(self)
    }
}
