use crate::{Document, IntoLiteral, Literal, Token};

use super::{Structure, TokenRef, Value};

pub(crate) trait IntoStructureInner {
    fn into_structure(self, doc: &mut Document, insert_after: TokenRef) -> Structure;
}

#[allow(private_bounds)]
pub trait IntoStructure: IntoStructureInner {}

impl<T: IntoStructureInner> IntoStructure for T {}

impl<T: IntoLiteral> IntoStructureInner for T {
    fn into_structure(self, doc: &mut Document, insert_after: TokenRef) -> Structure {
        let r = doc.insert_token_after(Token::Literal(self.into_literal()), insert_after);
        Structure::Value(Value::new(r))
    }
}

#[derive(Clone, Debug)]
pub struct ArrayBuilder(Vec<Literal>);

impl ArrayBuilder {
    pub fn new() -> Self { Self(vec![]) }
    pub fn push(&mut self, item: impl IntoLiteral) { self.0.push(item.into_literal()); }
}

pub struct ObjectBuilder<'a>(Vec<(Literal, Box<dyn IntoStructure + 'a>)>);

impl<'a> ObjectBuilder<'a> {
    pub fn new() -> Self { Self(vec![]) }
    pub fn push(&mut self, key: impl IntoLiteral, value: impl IntoStructure + 'a) {
        self.0.push((key.into_literal(), Box::new(value)));
    }
}

impl IntoStructureInner for ArrayBuilder {
    fn into_structure(self, _doc: &mut Document, _insert_after: TokenRef) -> Structure { todo!() }
}

impl<'a> IntoStructureInner for ObjectBuilder<'a> {
    fn into_structure(self, _doc: &mut Document, _insert_after: TokenRef) -> Structure { todo!() }
}
