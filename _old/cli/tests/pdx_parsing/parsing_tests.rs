use paradox_file::{Field, KeyVal, Object, Parser};
use crate::pdx_parsing::files::{COMMENTS_ETC, FULL_FILE_ADAL, INCORRECT_1, 
  INCORRECT_2, INCORRECT_3, OBJECTS, SIMPLE};

#[test]
fn simple_parse() {
  let mut parser = Parser::include_lexer(SIMPLE).unwrap();
  assert_eq!(parser.parse(), Ok(Object::new(vec![
    Field::new("width", KeyVal::new(6400_u16)),
    Field::new("height", KeyVal::new(2560_u16))
  ], 0)))
}

#[test]
fn objects_parse() {
  let mut parser = Parser::include_lexer(OBJECTS).unwrap();
  println!("{:#?}", parser.parse().unwrap());
}

#[test]
fn comments_etc_parse() {
  let mut parser = Parser::include_lexer(COMMENTS_ETC).unwrap();
  println!("{:#?}", parser.parse().unwrap());
}

#[test]
fn incorrect_1_parse() {
  let mut parser = Parser::include_lexer(INCORRECT_1).unwrap();
  println!("{}", parser.parse().unwrap_err());
}

#[test]
fn incorrect_2_parse() {
  let mut parser = Parser::include_lexer(INCORRECT_2).unwrap();
  println!("{}", parser.parse().unwrap_err());
}

#[test]
fn incorrect_3_parse() {
  let mut parser = Parser::include_lexer(INCORRECT_3).unwrap();
  println!("{}", parser.parse().unwrap_err());
}

#[test]
fn full_file_adal_parse() {
  let mut parser = Parser::include_lexer(FULL_FILE_ADAL).unwrap();
  println!("{:#?}", parser.parse());
}