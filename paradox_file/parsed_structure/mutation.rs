
use crate::{Field, FieldType, IntoLiteral, KeyVal, Object};

impl Object {
  pub fn get_child_objects_mut(&mut self) -> Vec<&mut Object> {
    self.fields.iter_mut().filter_map(|field| 
    if let FieldType::Object(obj) = &mut field.ft {
      Some(obj)
    } else { None }).collect()
  }
  
  /// returns true if field got successfully mutated
  pub fn mutate_kv<T: IntoLiteral>(&mut self, key: T, mutate: impl Fn(&mut KeyVal)) -> bool {
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
  pub fn insert_kv<KT: IntoLiteral, VT: IntoLiteral>(&mut self, index: usize, key: KT, value: VT) {
    self.fields.insert(index, Field::new(key, KeyVal::new(value)));
  }

  /// see [`Vec::retain`] for the implementation
  pub fn retain(&mut self, f: impl FnMut(&Field) -> bool) {
    self.fields.retain(f);
  }

  /// pushes a new key-value in an object
  pub fn push_kv<KT: IntoLiteral, VT: IntoLiteral>(&mut self, key: KT, value: VT) {
    self.fields.push(Field::new(key, KeyVal::new(value)));
  }
}

impl KeyVal {
  pub fn set_value<T: IntoLiteral>(&mut self, value: T) {
    self.value = value.into_literal();
  }
}