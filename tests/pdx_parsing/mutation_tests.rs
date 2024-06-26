use paradox_file::{Literal, Parser};
use crate::pdx_parsing::files::{OBJECTS, OBJECTS_CHANGED};

#[test]
pub fn objects_mutation() {
  let mut obj = Parser::include_lexer(OBJECTS).unwrap().parse().unwrap();

  obj.get_child_objects_mut()[0].mutate_key_val(
    Literal::String("event".to_string()),
    |kv| kv.set_value("making of the GHOSTING")
  );
  
  obj.get_child_objects_mut()[0].push_field_kv(
    Literal::String("status".to_string()),
    Literal::String("tired asf".to_string()),
  );

  println!("{}", OBJECTS_CHANGED);
  println!("{obj}");
  
  assert_eq!(obj.to_string(), OBJECTS_CHANGED);
}