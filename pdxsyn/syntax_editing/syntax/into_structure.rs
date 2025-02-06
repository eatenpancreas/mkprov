use crate::{Document, IntoLiteral, Literal, Token};

use super::{Array, Object, SealedObjectLike, SealedSyntaxLike, Structure, TokenRef, Value};

pub(crate) trait IntoStructureInner {
    fn into_structure(
        self: Box<Self>,
        doc: &mut Document,
        insert_after: TokenRef,
        indentation: usize,
    ) -> Structure;
}

#[allow(private_bounds)]
pub trait IntoStructure: IntoStructureInner {}

impl<T: IntoStructureInner> IntoStructure for T {}

impl<T: IntoLiteral> IntoStructureInner for T {
    fn into_structure(
        self: Box<Self>,
        doc: &mut Document,
        insert_after: TokenRef,
        _indentation: usize,
    ) -> Structure {
        let r = doc.insert_token_after(Token::Literal(self.into_literal()), insert_after);
        Structure::Value(Value::new(r))
    }
}

#[derive(Clone, Debug)]
pub struct ArrayBuilder(Vec<Literal>);

impl ArrayBuilder {
    pub fn new() -> Self { Self(vec![]) }
    pub fn push(&mut self, item: impl IntoLiteral) { self.0.push(item.into_literal()); }
    pub fn with(mut self, item: impl IntoLiteral) -> Self {
        self.push(item);
        self
    }
}

pub struct ObjectBuilder<'a>(Vec<(Literal, Box<dyn IntoStructure + 'a>)>);

impl<'a> ObjectBuilder<'a> {
    pub fn new() -> Self { Self(vec![]) }
    pub fn push(&mut self, key: impl IntoLiteral, value: impl IntoStructure + 'a) {
        self.0.push((key.into_literal(), Box::new(value)));
    }
    pub fn with(mut self, key: impl IntoLiteral, value: impl IntoStructure + 'a) -> Self {
        self.push(key, value);
        self
    }
}

impl IntoStructureInner for ArrayBuilder {
    fn into_structure(
        self: Box<Self>,
        doc: &mut Document,
        insert_after: TokenRef,
        indentation: usize,
    ) -> Structure {
        let opening = doc.insert_token_after(Token::BracketL, insert_after);
        let closure = doc.insert_token_after(Token::BracketR, opening);
        doc.insert_token_before(
            Token::Whitespace(format!("\n{}", "   ".repeat(indentation - 1))),
            closure,
        );
        let mut arr = Array::new_unclosed(opening, indentation);
        let indent = format!("\n{}", "   ".repeat(indentation));

        let mut tokens = vec![];
        for array_value in self.0 {
            tokens.push(Token::Whitespace(indent.clone()));
            tokens.push(Token::Literal(array_value));
        }

        let refs = doc.insert_tokens_after(tokens, opening);
        arr.raw_inner_mut()
            .extend(refs.into_iter().skip(1).step_by(2));
        arr.close(closure);
        Structure::Array(arr)
    }
}

impl<'a> IntoStructureInner for ObjectBuilder<'a> {
    fn into_structure(
        self: Box<Self>,
        doc: &mut Document,
        insert_after: TokenRef,
        indentation: usize,
    ) -> Structure {
        let opening = doc.insert_token_after(Token::BracketL, insert_after);
        let closure = doc.insert_token_after(Token::BracketR, opening);

        doc.insert_token_before(
            Token::Whitespace(format!("\n{}", "   ".repeat(indentation - 1))),
            closure,
        );

        let mut obj = Object::new_unclosed(opening, indentation);
        let indent = format!("\n{}", "   ".repeat(indentation));

        let mut last = opening;
        for (key, value) in self.0 {
            let tokens = vec![
                Token::Whitespace(indent.clone()),
                Token::Literal(key),
                Token::Whitespace(" ".to_owned()),
                Token::Equals,
                Token::Whitespace(" ".to_owned()),
            ];

            let refs = doc.insert_tokens_after(tokens, last);
            let assignment_end = *refs.last().unwrap();
            let val = value.into_structure(doc, assignment_end, indentation + 1);
            let (start, end) = val.token_range();
            last = end.unwrap_or(start);

            obj.raw_kvs_mut()
                .push((refs.into_iter().nth(1).unwrap(), val));
        }

        obj.close(closure);
        Structure::Object(obj)
    }
}
