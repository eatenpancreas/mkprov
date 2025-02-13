use crate::{Document, IntoLiteral, Literal};

use super::{DebugFmt, SealedSyntaxLike, TokenRef};

#[derive(Debug, Clone, PartialEq)]
pub struct Value(TokenRef);

impl SealedSyntaxLike for Value {
    fn token_range(&self) -> (TokenRef, Option<TokenRef>) { (*self.raw_inner(), None) }
}

impl Value {
    pub(crate) fn new(r: TokenRef) -> Self { Self(r) }
    pub(crate) fn raw_inner(&self) -> &TokenRef { &self.0 }

    pub fn get<'a>(&self, doc: &'a Document) -> Option<&'a Literal> {
        doc.get_literal(*self.raw_inner())
    }

    pub fn replace(&self, replace: impl IntoLiteral, doc: &mut Document) {
        if let Some(t) = doc.get_literal_mut(self.0) {
            *t = replace.into_literal();
        }
    }
}

impl DebugFmt for Value {
    fn debug_fmt(&self, doc: &Document) -> String {
        format!("{}", doc.get_token(*self.raw_inner()).unwrap())
    }
}
