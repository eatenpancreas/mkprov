use crate::{Date, Field, FieldType, IntoFieldType, IntoLiteral, KeyVal, Object};

impl Field {
  pub fn new<K: IntoLiteral, T: IntoFieldType>(key: K, field_type: T) -> Field {
    Field { key: key.into_literal(),  ft: field_type.into_field_type() }
  }

  pub fn new_literal<K: IntoLiteral>(key: K) -> Field {
    Field::new(key, FieldType::Literal)
  }
}

impl Object {
  pub fn new(
    vec: Vec<Field>, nesting: usize,
  ) -> Object {
    Object { fields: vec, nesting}
  }
}

impl Date {
  pub fn new(year: u16, month: u8, day: u8) -> Date {
    Date { year, month, day }
  }
}

impl KeyVal {
  pub fn new<T: IntoLiteral>(value: T) -> KeyVal {
    KeyVal {
      value: value.into_literal(),
    }
  }
}