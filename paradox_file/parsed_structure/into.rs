use crate::{Date, FieldType, KeyVal, Literal, Object};

pub trait IntoFieldType {
  fn into_field_type(self) -> FieldType;
}

pub trait IntoLiteral {
  fn into_literal(self) -> Literal;
}

impl IntoLiteral for Literal {
  fn into_literal(self) -> Literal {
    self
  }
}

impl IntoLiteral for u8 {
  fn into_literal(self) -> Literal {
    Literal::U8(self)
  }
}

impl IntoLiteral for u16 {
  fn into_literal(self) -> Literal {
    Literal::U16(self)
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

impl IntoLiteral for f32 {
  fn into_literal(self) -> Literal {
    Literal::F32(self)
  }
}

impl IntoFieldType for KeyVal {
  fn into_field_type(self) -> FieldType {
    FieldType::KeyVal(self)
  }
}

impl IntoFieldType for Literal {
  fn into_field_type(self) -> FieldType {
    FieldType::Literal
  }
}

impl IntoFieldType for Object {
  fn into_field_type(self) -> FieldType {
    FieldType::Object(self)
  }
}

impl IntoFieldType for FieldType {
  fn into_field_type(self) -> FieldType {
    self
  }
}