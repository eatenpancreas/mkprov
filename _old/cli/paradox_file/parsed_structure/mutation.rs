
use crate::{Field, FieldType, IntoLiteral, KeyVal, Object};

impl Object {
  pub fn get_child_objects_mut(&mut self) -> Vec<&mut Object> {
    self.fields.iter_mut().filter_map(|field| 
    if let FieldType::Object(obj) = &mut field.ft {
      Some(obj)
    } else { None }).collect()
  }
  
  /// returns true if field got successfully mutated
  pub fn mutate_kv<T: IntoLiteral>(
    &mut self, key: T, mutate: impl Fn(&mut KeyVal)
  ) -> bool {
    let key = key.into_literal();
    for field in &mut self.fields {
      if let FieldType::KeyVal(kv) = &mut field.ft {
        if field.key == key {
          mutate(kv);
          return true
        }
      }
    }
    false
  }

  /// pushes a new key-value in an object
  pub fn insert_kv<KT: IntoLiteral, VT: IntoLiteral>(
    &mut self, index: usize, key: KT, value: VT
  ) {
    self.fields.insert(index, Field::new(key, KeyVal::new(value)));
  }

  /// see [`Vec::retain`] for the implementation
  pub fn retain(&mut self, f: impl FnMut(&Field) -> bool) {
    self.fields.retain(f);
  }

  /// see [`core::slice::IterMut::find`] for the implementation
  pub fn find_mut(
    &mut self, f: impl FnMut(&&mut Field) -> bool
  ) -> Option<&mut Field> {
    self.fields.iter_mut().find(f)
  }

  /// see [`Vec::push`] for the implementation
  pub fn push(&mut self, f: Field) {
    self.fields.push(f);
  }

  /// pushes a new key-value in an object
  pub fn push_kv<KT: IntoLiteral, VT: IntoLiteral>(
    &mut self, key: KT, value: VT
  ) {
    self.fields.push(Field::new(key, KeyVal::new(value)));
  }

  pub fn is_literal_array(&self) -> bool {
    let mut is_lit_array = true;

    for field in &self.fields {
      if field.ft != FieldType::Literal {
        is_lit_array = false
      }
    }

    is_lit_array
  }

  pub fn len(&self) -> usize {
    self.fields.len()
  }
}

impl KeyVal {
  pub fn set_value<T: IntoLiteral>(&mut self, value: T) {
    self.value = value.into_literal();
  }
}