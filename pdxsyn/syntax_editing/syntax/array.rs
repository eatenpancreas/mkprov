use itertools::Itertools;

use super::{SealedSyntaxLike, TokenRef};
use crate::Document;

#[derive(Debug, Clone)]
pub struct Array {
    opening: TokenRef,
    closure: TokenRef,
    values: Vec<TokenRef>,
    depth: usize,
}

impl SealedSyntaxLike for Array {
    fn token_range(&self) -> (TokenRef, Option<TokenRef>) { (self.opening, Some(self.closure)) }
}

impl Array {
    pub(crate) fn new_unclosed(opening: TokenRef, depth: usize) -> Self {
        Self { opening, closure: opening, values: vec![], depth }
    }

    pub(crate) fn close(&mut self, closure: TokenRef) { self.closure = closure; }
    pub(crate) fn raw_inner(&self) -> &Vec<TokenRef> { &self.values }
    pub(crate) fn raw_inner_mut(&mut self) -> &mut Vec<TokenRef> { &mut self.values }

    pub fn debug_fmt(&self, doc: &Document) -> String {
        let tabbing = format!("\n{}", "  ".repeat(self.depth));
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
            .then_some(format!("\n{}", "  ".repeat(self.depth - 1)))
            .unwrap_or("".to_string());

        format!("[{beginln}{contents}{endln}]")
    }
}
