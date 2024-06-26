
use crate::{Field, FieldType, IntoLiteral, KeyVal, Literal, Object};

impl Object {
  pub fn get_child_objects_mut(&mut self) -> Vec<&mut Object> {
    self.fields.iter_mut().filter_map(|field| 
    if let FieldType::Object(obj) = &mut field.ft {
      Some(obj)
    } else { None }).collect()
  }
  
  pub fn mutate_key_val(&mut self, key: Literal, mutate: impl Fn(&mut KeyVal)) {
    for field in &mut self.fields {
      if let FieldType::KeyVal(kv) = &mut field.ft {
        if field.key == key {
          mutate(kv);
        }
      }
    }
  }

  pub fn push_field_kv(&mut self, key: Literal, value: Literal) {
    self.fields.push(Field::new(key, KeyVal::new(value)));
  }
}

impl KeyVal {
  pub fn set_value<T: IntoLiteral>(&mut self, value: T) {
    self.value = value.into_literal();
  }
}