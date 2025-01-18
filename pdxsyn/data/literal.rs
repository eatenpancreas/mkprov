use derived_deref::{Deref, DerefMut};
use thiserror::Error;

use super::Date;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    I64(i64),
    F32(f32, Precision),
    String(String),
    ExplicitString(String),
    Date(Date),
}

impl Literal {
    pub fn explicit_string(string: String) -> Self {
        Self::ExplicitString(string)
    }
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

pub(crate) trait AsLiteral: Copy {
    fn as_literal(self) -> Literal;
}

pub trait IntoLiteral {
    fn into_literal(self) -> Literal;
}

impl<T: AsLiteral> AsLiteral for &T {
    fn as_literal(self) -> Literal {
        (*self).as_literal()
    }
}

impl<T: AsLiteral> IntoLiteral for T {
    fn into_literal(self) -> Literal {
        self.as_literal()
    }
}

impl AsLiteral for u8 {
    fn as_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl AsLiteral for u16 {
    fn as_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl AsLiteral for u32 {
    fn as_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl AsLiteral for i8 {
    fn as_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl AsLiteral for i16 {
    fn as_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl AsLiteral for i32 {
    fn as_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl AsLiteral for i64 {
    fn as_literal(self) -> Literal {
        Literal::I64(self as i64)
    }
}

impl AsLiteral for Date {
    fn as_literal(self) -> Literal {
        Literal::Date(self)
    }
}

impl AsLiteral for &str {
    fn as_literal(self) -> Literal {
        Literal::String(self.to_string())
    }
}

impl IntoLiteral for String {
    fn into_literal(self) -> Literal {
        Literal::String(self)
    }
}

impl AsLiteral for &Literal {
    fn as_literal(self) -> Literal {
        self.clone()
    }
}

impl IntoLiteral for Literal {
    fn into_literal(self) -> Literal {
        self
    }
}

impl AsLiteral for f32 {
    fn as_literal(self) -> Literal {
        let num_str = self.to_string();

        Literal::F32(self, Precision::new(num_str.split('.').nth(1).map(|d| d.len()).unwrap_or(0)))
    }
}
