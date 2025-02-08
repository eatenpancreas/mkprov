use itertools::Itertools;

use super::{ArrayBuilder, DebugFmt, SealedSyntaxLike, TokenRef};
use crate::{Document, IntoLiteral, Literal, Token};

#[derive(Debug, Clone, PartialEq)]
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

    /// Removes the item at `index`
    pub fn remove(&mut self, doc: &mut Document, index: usize) -> Option<Literal> {
        if index >= self.raw_inner().len() {
            return None;
        }

        let item = self.raw_inner_mut().remove(index);
        if let Some(left) = doc.token_sub_position(item, 1) {
            if doc.get_token(left).is_some_and(|t| t.is_whitespace()) {
                doc.remove_token(left);
            }
        }

        doc.remove_token(item)
            .map(|t| t.into_literal().expect("Expected literal"))
    }

    pub fn extend(&mut self, doc: &mut Document, items: ArrayBuilder) {
        items
            .0
            .into_iter()
            .map(|item| self.push(doc, item))
            .for_each(|_| {});
    }

    pub fn push(&mut self, doc: &mut Document, item: impl IntoLiteral) {
        let last = *self.raw_inner().last().unwrap_or(&self.opening);
        let indent = "   ".repeat(self.depth);
        let inserted = doc.insert_tokens_after(
            vec![Token::Whitespace(format!("\n{indent}")), Token::Literal(item.into_literal())],
            last,
        );

        self.raw_inner_mut().push(inserted[1]);
    }

    /// Gets the item at `index`
    pub fn get<'a>(&self, doc: &'a Document, index: usize) -> Option<&'a Literal> {
        doc.get_literal(*self.raw_inner().get(index)?)
    }

    pub fn len(&self) -> usize { self.raw_inner().len() }

    /// Checks if the array includes `T`
    pub fn has<T: PartialEq<Literal>>(&self, doc: &Document, has: T) -> bool {
        self.raw_inner()
            .iter()
            .any(|r| (&has == doc.get_literal(*r).expect("Expected Literal")))
    }

    /// Gets the indices of anything matching `search`
    pub fn indices_of<T: PartialEq<Literal>>(&self, doc: &Document, search: T) -> Vec<usize> {
        self.raw_inner()
            .iter()
            .enumerate()
            .filter_map(|(i, r)| {
                (&search == doc.get_literal(*r).expect("Expected Literal")).then_some(i)
            })
            .collect()
    }

    /// Iterates over the fields of the array
    pub fn iter<'a>(&'a self, doc: &'a Document) -> impl Iterator<Item = &'a Literal> {
        self.raw_inner()
            .iter()
            .map(move |r| doc.get_literal(*r).expect("Expected literal"))
    }
}

impl DebugFmt for Array {
    fn debug_fmt(&self, doc: &Document) -> String {
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
