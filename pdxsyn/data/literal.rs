use derived_deref::{Deref, DerefMut};
use thiserror::Error;

use crate::is_valid_ident;

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
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Literal::I64(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_f32(&self) -> Option<(f32, Precision)> {
        match self {
            Literal::F32(i, precision) => Some((*i, *precision)),
            _ => None,
        }
    }
    pub fn as_date(&self) -> Option<Date> {
        match self {
            Literal::Date(d) => Some(*d),
            _ => None,
        }
    }
    pub fn as_any_string(&self) -> Option<&String> {
        match self {
            Literal::String(s) => Some(s),
            Literal::ExplicitString(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Precision(usize);

impl Precision {
    #[inline]
    pub fn new(precision: usize) -> Self { Self(precision) }
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
    fn as_literal(self) -> Literal { (*self).as_literal() }
}

impl<T: AsLiteral> IntoLiteral for T {
    fn into_literal(self) -> Literal { self.as_literal() }
}

impl AsLiteral for u8 {
    fn as_literal(self) -> Literal { Literal::I64(self as i64) }
}

impl PartialEq<Literal> for u8 {
    fn eq(&self, other: &Literal) -> bool { other.as_i64().is_some_and(|s| s == *self as i64) }
}

impl AsLiteral for u16 {
    fn as_literal(self) -> Literal { Literal::I64(self as i64) }
}

impl PartialEq<Literal> for u16 {
    fn eq(&self, other: &Literal) -> bool { other.as_i64().is_some_and(|s| s == *self as i64) }
}

impl AsLiteral for u32 {
    fn as_literal(self) -> Literal { Literal::I64(self as i64) }
}

impl PartialEq<Literal> for u32 {
    fn eq(&self, other: &Literal) -> bool { other.as_i64().is_some_and(|s| s == *self as i64) }
}

impl AsLiteral for i8 {
    fn as_literal(self) -> Literal { Literal::I64(self as i64) }
}

impl PartialEq<Literal> for i8 {
    fn eq(&self, other: &Literal) -> bool { other.as_i64().is_some_and(|s| s == *self as i64) }
}

impl AsLiteral for i16 {
    fn as_literal(self) -> Literal { Literal::I64(self as i64) }
}

impl PartialEq<Literal> for i16 {
    fn eq(&self, other: &Literal) -> bool { other.as_i64().is_some_and(|s| s == *self as i64) }
}

impl AsLiteral for i32 {
    fn as_literal(self) -> Literal { Literal::I64(self as i64) }
}

impl PartialEq<Literal> for i32 {
    fn eq(&self, other: &Literal) -> bool { other.as_i64().is_some_and(|s| s == *self as i64) }
}

impl AsLiteral for i64 {
    fn as_literal(self) -> Literal { Literal::I64(self as i64) }
}

impl PartialEq<Literal> for i64 {
    fn eq(&self, other: &Literal) -> bool { other.as_i64().is_some_and(|s| s == *self) }
}

impl AsLiteral for f32 {
    fn as_literal(self) -> Literal {
        let num_str = self.to_string();

        Literal::F32(self, Precision::new(num_str.split('.').nth(1).map(|d| d.len()).unwrap_or(0)))
    }
}

impl AsLiteral for Date {
    fn as_literal(self) -> Literal { Literal::Date(self) }
}

impl PartialEq<Literal> for Date {
    fn eq(&self, other: &Literal) -> bool { other.as_date().is_some_and(|s| s == *self) }
}

impl AsLiteral for &str {
    fn as_literal(self) -> Literal { self.to_string().into_literal() }
}

impl PartialEq<Literal> for &str {
    fn eq(&self, other: &Literal) -> bool { other.as_any_string().is_some_and(|s| s == self) }
}

impl IntoLiteral for String {
    fn into_literal(self) -> Literal {
        let mut chars = self.chars();
        if chars.all(|c| is_valid_ident(c)) {
            Literal::String(self)
        } else {
            Literal::ExplicitString(self)
        }
    }
}

impl PartialEq<Literal> for String {
    fn eq(&self, other: &Literal) -> bool { other.as_any_string().is_some_and(|s| s == self) }
}

impl AsLiteral for &Literal {
    fn as_literal(self) -> Literal { self.clone() }
}

impl IntoLiteral for Literal {
    fn into_literal(self) -> Literal { self }
}
