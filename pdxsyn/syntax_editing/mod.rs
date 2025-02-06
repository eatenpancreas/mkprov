pub mod parsing;
pub mod syntax;

use derived_deref::Deref;
use itertools::Itertools;

use crate::{Literal, Token};

#[derive(Clone, Copy, Debug, Deref, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TokenRef(usize);

/// `Document` is an ordered reference for the shadow data that gets parsed out
/// of its `parse` method. It maintains a mapping of tokens and their order,
/// allowing for efficient retrieval, modification, and removal of tokens.
#[derive(Clone, Debug)]
pub struct Document {
    token_idx: usize,
    inner_tokens: Vec<(TokenRef, Token)>,
    desired_line_wrap: usize,
}

impl Document {
    pub fn create(tokens: Vec<Token>) -> Self {
        let mut token_idx = 0;
        let tokens = tokens.into_iter().map(|t| {
            let token_ref = TokenRef(token_idx);
            token_idx += 1;
            (token_ref, t)
        });
        Self { inner_tokens: tokens.collect(), token_idx, desired_line_wrap: 1 }
    }

    pub fn with_line_wrap(&mut self, wrap: usize) { self.desired_line_wrap = wrap }

    pub(crate) fn insert_token_before(&mut self, t: Token, before: TokenRef) -> TokenRef {
        let token_ref = TokenRef(self.token_idx);
        self.token_idx += 1;
        let pos = self.token_position(before).unwrap_or(0);
        self.inner_tokens.insert(pos, (token_ref, t));
        token_ref
    }

    pub(crate) fn insert_token_after(&mut self, t: Token, after: TokenRef) -> TokenRef {
        let token_ref = TokenRef(self.token_idx);
        self.token_idx += 1;
        let pos = self.token_position(after).unwrap_or(0) + 1;
        self.inner_tokens.insert(pos, (token_ref, t));
        token_ref
    }

    pub(crate) fn insert_tokens_after(
        &mut self,
        tokens: impl IntoIterator<Item = Token>,
        after: TokenRef,
    ) -> Vec<TokenRef> {
        let mut tokens = tokens.into_iter().collect_vec();
        tokens.reverse();

        let mut refs = tokens
            .into_iter()
            .map(move |t| self.insert_token_after(t, after))
            .collect_vec();

        refs.reverse();
        refs
    }

    pub(crate) fn insert_tokens_before(
        &mut self,
        tokens: impl IntoIterator<Item = Token>,
        before: TokenRef,
    ) -> Vec<TokenRef> {
        tokens
            .into_iter()
            .map(move |t| self.insert_token_before(t, before))
            .collect_vec()
    }

    pub(crate) fn get_literal(&self, r: TokenRef) -> Option<&Literal> {
        self.get_token(r)?.as_literal()
    }

    pub(crate) fn _get_literal_mut(&mut self, r: TokenRef) -> Option<&mut Literal> {
        self._get_token_mut(r)?.as_literal_mut()
    }

    pub(crate) fn get_token(&self, r: TokenRef) -> Option<&Token> {
        self.inner_tokens
            .iter()
            .find_map(|(r2, t)| (r == *r2).then_some(t))
    }

    pub(crate) fn _get_token_mut(&mut self, r: TokenRef) -> Option<&mut Token> {
        self.inner_tokens
            .iter_mut()
            .find_map(|(r2, t)| (r == *r2).then_some(t))
    }

    pub(crate) fn token_position(&self, r: TokenRef) -> Option<usize> {
        self.inner_tokens.iter().position(|(r2, _)| *r2 == r)
    }

    pub(crate) fn token_at(&self, pos: usize) -> Option<TokenRef> {
        self.inner_tokens.iter().nth(pos).map(|(r, _)| *r)
    }

    /// Gets the ref for the given token position minus the `sub`
    pub(crate) fn token_sub_position(&self, r: TokenRef, sub: usize) -> Option<TokenRef> {
        self.token_at(self.token_position(r).unwrap().checked_sub(sub)?)
    }

    /// Gets the ref for the given token position plus the `sub`
    pub(crate) fn _token_add_position(&self, r: TokenRef, add: usize) -> Option<TokenRef> {
        self.token_at(self.token_position(r).unwrap().checked_add(add)?)
    }

    pub(crate) fn remove_token(&mut self, r: TokenRef) -> Option<Token> {
        Some(self.inner_tokens.remove(self.token_position(r)?).1)
    }

    pub(crate) fn remove_range(&mut self, left: TokenRef, right: TokenRef) {
        let left = self.token_position(left).unwrap();
        let right = self.token_position(right).unwrap();

        for _ in 0..=right - left {
            self.token_at(left).and_then(|t| self.remove_token(t));
        }
    }

    /// Returns all tokens in the document in their original order, forming a single string. Consumes `self`
    pub fn into_string(self) -> String {
        self.return_tokens().iter().map(|t| t.to_string()).join("")
    }

    /// Returns all tokens in the document in their original order. Consumes `self`
    pub fn return_tokens(self) -> Vec<Token> {
        self.inner_tokens.into_iter().map(|(_, t)| t).collect()
    }
}
