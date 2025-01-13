use derived_deref::{Deref, DerefMut};
use thiserror::Error;

use super::Date;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    I64(i64),
    F32(f32, Precision),
    String(String),
    Date(Date),
}

#[derive(Debug, Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Precision(usize);

impl Precision {
    #[inline]
    pub fn new(precision: usize) -> Self {
        Self(precision)
    }
}

#[derive(Error, Debug)]
pub enum ParseNumericError {
    #[error("Invalid numeric value: {0}")]
    InvalidNumericType(String),
}

pub trait IntoLiteral {
    fn into_literal(self) -> Literal;
}

impl IntoLiteral for u8 {
    fn into_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl IntoLiteral for u16 {
    fn into_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl IntoLiteral for u32 {
    fn into_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl IntoLiteral for i8 {
    fn into_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl IntoLiteral for i16 {
    fn into_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl IntoLiteral for i32 {
    fn into_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl IntoLiteral for i64 {
    fn into_literal(self) -> Literal {
        Literal::I64(self as i64)
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
        let num_str = self.to_string();

        Literal::F32(
            self,
            Precision::new(num_str.split('.').nth(1).map(|d| d.len()).unwrap_or(0)),
        )
    }
}
