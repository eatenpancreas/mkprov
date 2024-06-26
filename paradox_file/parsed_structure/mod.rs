mod into;
mod mutation;
mod to_string;
mod instancing;

pub use into::*;
use crate::Literal;

#[derive(Debug, PartialEq)]
pub struct Field {
  key: Literal,
  ft: FieldType,
}

#[derive(Debug, PartialEq)]
pub enum FieldType {
  KeyVal(KeyVal),
  Literal,
  Object(Object),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Location(pub(crate) usize);

#[derive(Debug, PartialEq)]
pub struct Object {
  fields: Vec<Field>,
  nesting: usize,
}

#[derive(Debug, PartialEq)]
pub struct Date {
  year: u16,
  month: u8,
  day: u8,
}

#[derive(Debug, PartialEq)]
pub struct KeyVal {
  value: Literal,
}
