mod parser;
use parser::*;

use super::{Document, TokenRef, syntax::*};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseDocumentError {
    #[error("Unexpected token {0} at pos {1}, expected {2}")]
    UnexpectedToken(String, usize, &'static str),
    #[error("Unexpected end of file")]
    UnexpectedEnd,
    #[error("Unexpected end of object at pos {0}")]
    EarlyBracket(usize),
    #[error("Mixed structure detected - mixing array's and objects are not supported. at pos {0}")]
    MixedStructure(usize),
}

impl Document {
    pub fn parse(&self) -> Result<RootObject, ParseDocumentError> {
        let mut parser = DocumentParser::new(self);
        let mut root = RootObject::new();

        while let Some((token_ref, _)) = parser.pop_until_expected(
            |t| match t {
                _ if t.is_literal() => ExpectAction::Valid(()),
                _ if t.is_delimiter() => ExpectAction::Ignore,
                _ => ExpectAction::Invalid,
            },
            "identifier",
        )? {
            parser
                .pop_until_expected(
                    |t| match t {
                        _ if t.is_equals() => ExpectAction::Valid(()),
                        _ if t.is_delimiter() => ExpectAction::Ignore,
                        _ => ExpectAction::Invalid,
                    },
                    "=",
                )?
                .ok_or(ParseDocumentError::UnexpectedEnd)?;

            let struc = parser.parse_structure()?;
            root.raw_kvs_mut().push((token_ref, struc));
        }

        Ok(root)
    }
}

impl<'a> DocumentParser<'a> {
    fn parse_structure(&mut self) -> Result<Structure, ParseDocumentError> {
        let (ref_1, is_bracket_opening) = self
            .pop_until_expected(
                |t| match t {
                    _ if t.is_literal() => ExpectAction::Valid(false),
                    _ if t.is_bracket_l() => ExpectAction::Valid(true),
                    _ if t.is_delimiter() => ExpectAction::Ignore,
                    _ => ExpectAction::Invalid,
                },
                "word, string or {",
            )?
            .ok_or(ParseDocumentError::UnexpectedEnd)?;

        if !is_bracket_opening {
            return Ok(Structure::Value(Value::new(ref_1)));
        }

        // found {: here goes advanced structure parsing
        let mut structure = None;

        while let Some((ref_2, is_bracket_closing)) = self.pop_until_expected(
            |t| match t {
                _ if t.is_literal() => ExpectAction::Valid(false),
                _ if t.is_bracket_r() => ExpectAction::Valid(true),
                _ if t.is_delimiter() => ExpectAction::Ignore,
                _ => ExpectAction::Invalid,
            },
            "word, string or }",
        )? {
            if is_bracket_closing && structure.is_some() {
                let mut structure = structure.unwrap();

                match &mut structure {
                    Structure::Object(obj) => obj.close(ref_2),
                    Structure::Array(obj) => obj.close(ref_2),
                    _ => {}
                }

                return Ok(structure);
            } else if is_bracket_closing {
                let mut empty_arr = Array::new_unclosed(ref_1);
                empty_arr.close(ref_2);
                return Ok(Structure::Array(empty_arr));
            } else {
                let expect_object = self
                    .peek_until_expected(
                        |t| match t {
                            _ if t.is_equals() => ExpectAction::Valid(()),
                            _ if t.is_delimiter() => ExpectAction::Ignore,
                            _ => ExpectAction::End,
                        },
                        "",
                    )?
                    .is_some();

                match (expect_object, structure.as_mut()) {
                    (true, Some(Structure::Object(obj))) => {
                        self.parse_object(obj, ref_2)?;
                    }
                    (false, Some(Structure::Array(arr))) => {
                        arr.raw_inner_mut().push(ref_2);
                    }
                    (true, None) => {
                        let mut obj = Object::new_unclosed(ref_1);
                        self.parse_object(&mut obj, ref_2)?;
                        structure = Some(Structure::Object(obj))
                    }
                    (false, None) => {
                        let mut arr = Array::new_unclosed(ref_1);
                        arr.raw_inner_mut().push(ref_2);
                        structure = Some(Structure::Array(arr));
                    }
                    _ => {
                        return Err(ParseDocumentError::MixedStructure(
                            self.document().token_position(ref_2).unwrap_or(0),
                        ));
                    }
                }
            }
        }

        // panic!("could not make structure");
        Err(ParseDocumentError::UnexpectedEnd)
    }

    fn parse_object(
        &mut self,
        object: &mut Object,
        key_ref: TokenRef,
    ) -> Result<(), ParseDocumentError> {
        self.pop_until_expected(
            |t| match t {
                _ if t.is_equals() => ExpectAction::Valid(()),
                _ if t.is_delimiter() => ExpectAction::Ignore,
                _ => ExpectAction::Invalid,
            },
            "=",
        )?
        .ok_or(ParseDocumentError::UnexpectedEnd)?;

        let value = self.parse_structure()?;
        object.raw_kvs_mut().push((key_ref, value));

        Ok(())
    }
}
