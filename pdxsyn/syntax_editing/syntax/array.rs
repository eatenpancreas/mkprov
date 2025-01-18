use itertools::Itertools;

use super::{DocumentRef, SealedSyntaxLike};
use crate::Document;

#[derive(Debug, Clone)]
pub struct Array {
    opening: DocumentRef,
    closure: DocumentRef,
    values: Vec<DocumentRef>,
}

impl SealedSyntaxLike for Array {
    fn token_range(&self) -> (DocumentRef, Option<DocumentRef>) {
        (self.opening, Some(self.closure))
    }
}

impl Array {
    pub(crate) fn new_unclosed(opening: DocumentRef) -> Self {
        Self { opening, closure: opening, values: vec![] }
    }

    pub(crate) fn close(&mut self, closure: DocumentRef) { self.closure = closure; }
    pub(crate) fn raw_inner(&self) -> &Vec<DocumentRef> { &self.values }
    pub(crate) fn raw_inner_mut(&mut self) -> &mut Vec<DocumentRef> { &mut self.values }

    pub(crate) fn debug_fmt_inner(&self, doc: &Document, nesting: usize) -> String {
        let tabbing = format!("\n{}", "  ".repeat(nesting));
        let contents = self
            .raw_inner()
            .iter()
            .format_with(tabbing.as_str(), |d_ref, f| {
                f(&format_args!("{}", doc.get_token(*d_ref).unwrap()))
            });

        let beginln = (self.raw_inner().len() >= 1)
            .then_some(&*tabbing)
            .unwrap_or("");

        let endln = (self.raw_inner().len() >= 1)
            .then_some(format!("\n{}", "  ".repeat(nesting - 1)))
            .unwrap_or("".to_string());

        format!("[{beginln}{contents}{endln}]")
    }

    pub fn debug_fmt(&self, doc: &Document) -> String { self.debug_fmt_inner(doc, 1) }
}
