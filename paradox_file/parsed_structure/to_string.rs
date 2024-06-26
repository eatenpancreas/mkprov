use std::fmt::{Display, Formatter};
use crate::{Date, FieldType, Literal, Location, Object};

impl Object {
  fn padding(&self) -> String {
    let mut padding = String::new();
    for _ in 0..self.nesting * 4 {
      padding.push(' ')
    }
    padding
  }
}

impl Display for Object {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut str = String::new();
    let padding = self.padding();
    let lit_arr = self.is_literal_array();
    let mut i = 0;

    for field in &self.fields {
      if lit_arr && i % 4 != 0 {
        str.push(' ');
      } else {
        str.push_str(padding.as_str());
      }
      str.push_str(field.key.to_string().as_str());

      match &field.ft {
        FieldType::Literal => {}
        FieldType::KeyVal(kv) => {
          str.push_str(" = ");
          str.push_str(kv.value.to_string().as_str());
        }
        FieldType::Object(obj) => {
          str.push_str(" = {\n");
          str.push_str(obj.to_string().as_str());
          str.push_str("\n}\n");
        }
      }

      if !lit_arr || i % 4 == 3 {
        str.push('\n');
      }
      i+=1;
    }


    write!(f, "{str}")
  }
}

impl Display for Literal {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      Literal::U8(x) => x.to_string(),
      Literal::U16(x) => x.to_string(),
      Literal::F32(x) => x.to_string(),
      Literal::String(x) => {
        if x.contains(|x: char| x.is_whitespace()) {
          format!(r#""{x}""#)
        } else {
          x.to_string()
        }
      },
      Literal::Date(x) => x.to_string()
    };
    write!(f, "{str}")
  }
}

impl Display for Location {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "@ char {}", self.0)
  }
}

impl Display for Date {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:0>4}.{:0>2}.{:0>2}", self.year, self.month, self.day)
  }
}