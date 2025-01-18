use crate::{Document, Literal};

use super::{DocumentRef, SealedSyntaxLike};

#[derive(Debug, Clone)]
pub struct Value(DocumentRef);

impl SealedSyntaxLike for Value {
    fn token_range(&self) -> (DocumentRef, Option<DocumentRef>) { (*self.raw_inner(), None) }
}

impl Value {
    pub(crate) fn new(r: DocumentRef) -> Self { Self(r) }
    pub(crate) fn raw_inner(&self) -> &DocumentRef { &self.0 }

    pub fn get<'a>(&self, doc: &'a Document) -> Option<&'a Literal> {
        doc.get_literal(*self.raw_inner())
    }

    pub fn debug_fmt(&self, doc: &Document) -> String {
        format!("{}", doc.get_token(*self.raw_inner()).unwrap())
    }
}
