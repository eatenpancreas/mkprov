use paradox_file::{Literal, Parser};
use crate::pdx_parsing::files::{FULL_UNC_FILE, OBJECTS, OBJECTS_CHANGED};

#[test]
pub fn objects_mutation() {
  let mut obj = Parser::include_lexer(OBJECTS).unwrap().parse().unwrap();

  obj.get_child_objects_mut()[0].mutate_kv(
    Literal::String("event".to_string()),
    |kv| kv.set_value("making of the GHOSTING")
  );
  
  obj.get_child_objects_mut()[0].push_kv(
    Literal::String("status".to_string()),
    Literal::String("tired asf".to_string()),
  );

  println!("{}", OBJECTS_CHANGED);
  println!("{obj}");
  
  assert_eq!(obj.to_string(), OBJECTS_CHANGED);
}


#[test]
pub fn full_unc_mutation() {
  let mut obj = Parser::include_lexer(FULL_UNC_FILE).unwrap().parse().unwrap();
  println!("{obj}");

  if !obj.mutate_kv("owner",
    |kv| kv.set_value("AAH")) {
    obj.insert_kv(0, "owner", "AAH")
  }

  if !obj.mutate_kv("controller",
    |kv| kv.set_value("AAH")) {
    obj.insert_kv(1, "controller", "AAH")
  }
  
  obj.retain(|field| !field.key_is("native_size") 
    && !field.key_is("native_ferocity") 
    && !field.key_is("native_hostileness")
  );

  println!("{obj}");

  if !obj.mutate_kv("owner",
    |kv| kv.set_value("BOH")) {
    obj.insert_kv(0,"owner", "BOH")
  }

  if !obj.mutate_kv("controller",
    |kv| kv.set_value("BOH")) {
    obj.insert_kv(1,"controller", "BOH")
  }

  println!("{obj}");
}