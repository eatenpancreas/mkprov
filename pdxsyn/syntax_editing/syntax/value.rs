use crate::{Document, Literal};

use super::{DebugFmt, SealedSyntaxLike, TokenRef};

#[derive(Debug, Clone)]
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
}

impl DebugFmt for Value {
    fn debug_fmt(&self, doc: &Document) -> String {
        format!("{}", doc.get_token(*self.raw_inner()).unwrap())
    }
}
