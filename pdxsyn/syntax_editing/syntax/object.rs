use itertools::Itertools;

use super::{SealedObjectLike, SealedSyntaxLike, Structure, TokenRef};
use crate::Document;

#[derive(Debug, Clone)]
pub struct Object {
    opening: TokenRef,
    closure: TokenRef,
    values: Vec<(TokenRef, Structure)>,
    depth: usize,
}

impl SealedSyntaxLike for Object {
    fn token_range(&self) -> (TokenRef, Option<TokenRef>) { (self.opening, Some(self.closure)) }
}

impl SealedObjectLike for Object {
    fn raw_kvs(&self) -> &Vec<(TokenRef, Structure)> { &self.values }
    fn raw_kvs_mut(&mut self) -> &mut Vec<(TokenRef, Structure)> { &mut self.values }
    fn indentation(&self) -> usize { self.depth }
}

impl Object {
    pub(crate) fn new_unclosed(opening: TokenRef, depth: usize) -> Self {
        Self { opening, closure: opening, values: vec![], depth }
    }

    pub(crate) fn close(&mut self, closure: TokenRef) { self.closure = closure; }

    pub fn debug_fmt(&self, doc: &Document) -> String {
        let tabbing = format!("\n{}", "  ".repeat(self.depth));
        let contents = self
            .raw_kvs()
            .iter()
            .format_with(tabbing.as_str(), |(d_ref, s), f| {
                f(&format_args!("{} = {}", doc.get_token(*d_ref).unwrap(), s.debug_fmt(doc)))
            });

        let beginln = (self.raw_kvs().len() >= 1)
            .then_some(&*tabbing)
            .unwrap_or("");

        let endln = (self.raw_kvs().len() >= 1)
            .then_some(format!("\n{}", "  ".repeat(self.depth - 1)))
            .unwrap_or("".to_string());

        format!("{{{beginln}{contents}{endln}}}")
    }
}
