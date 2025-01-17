use itertools::Itertools;

use super::{DocumentRef, SealedObjectLike, SealedSyntaxLike, Structure};
use crate::Document;

#[derive(Debug, Clone)]
pub struct Object {
    opening: DocumentRef,
    closure: DocumentRef,
    values: Vec<(DocumentRef, Structure)>,
}

impl SealedSyntaxLike for Object {
    fn token_range(&self) -> (DocumentRef, Option<DocumentRef>) {
        (self.opening, Some(self.closure))
    }
}

impl SealedObjectLike for Object {
    fn raw_kvs(&self) -> &Vec<(DocumentRef, Structure)> {
        &self.values
    }

    fn raw_kvs_mut(&mut self) -> &mut Vec<(DocumentRef, Structure)> {
        &mut self.values
    }
}

impl Object {
    pub(crate) fn new_unclosed(opening: DocumentRef) -> Self {
        Self {
            opening,
            closure: opening,
            values: vec![],
        }
    }

    pub(crate) fn close(&mut self, closure: DocumentRef) {
        self.closure = closure;
    }

    pub(crate) fn debug_fmt_inner(&self, doc: &Document, nesting: usize) -> String {
        let tabbing = format!("\n{}", "\t".repeat(nesting));
        let contents = self
            .raw_kvs()
            .iter()
            .format_with(tabbing.as_str(), |(d_ref, s), f| {
                f(&format_args!(
                    "{} = {}",
                    doc.get_token(*d_ref).unwrap(),
                    s.debug_fmt_inner(doc, nesting)
                ))
            });

        let beginln = (self.raw_kvs().len() >= 1)
            .then_some(&*tabbing)
            .unwrap_or("");
        let endln = (self.raw_kvs().len() >= 1).then_some("\n").unwrap_or("");

        format!("{{{beginln}{contents}{endln}}}")
    }

    pub fn debug_fmt(&self, doc: &Document) -> String {
        self.debug_fmt_inner(doc, 0)
    }
}
