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
}

impl Document {
    pub fn create(tokens: Vec<Token>) -> Self {
        let mut token_idx = 0;
        let tokens = tokens.into_iter().map(|t| {
            let token_ref = TokenRef(token_idx);
            token_idx += 1;
            (token_ref, t)
        });
        Self { inner_tokens: tokens.collect(), token_idx }
    }

    pub(crate) fn insert_token_at(&mut self, t: Token, pos: usize) -> TokenRef {
        let token_ref = TokenRef(self.token_idx);
        self.token_idx += 1;
        self.inner_tokens.insert(pos, (token_ref, t));
        token_ref
    }

    pub(crate) fn get_literal(&self, r: TokenRef) -> Option<&Literal> {
        self.get_token(r)?.as_literal()
    }

    pub(crate) fn get_literal_mut(&mut self, r: TokenRef) -> Option<&mut Literal> {
        self.get_token_mut(r)?.as_literal_mut()
    }

    pub(crate) fn get_token(&self, r: TokenRef) -> Option<&Token> {
        self.get_token_at(self.token_position(r)?)
    }

    pub(crate) fn get_token_mut(&mut self, r: TokenRef) -> Option<&mut Token> {
        self.get_mut_token_at(self.token_position(r)?)
    }

    pub(crate) fn token_position(&self, r: TokenRef) -> Option<usize> {
        self.inner_tokens.iter().position(|(r2, _)| *r2 == r)
    }

    pub(crate) fn get_token_at(&self, pos: usize) -> Option<&Token> {
        Some(&self.inner_tokens.get(pos)?.1)
    }

    pub(crate) fn get_mut_token_at(&mut self, pos: usize) -> Option<&mut Token> {
        Some(&mut self.inner_tokens.get_mut(pos)?.1)
    }

    pub(crate) fn remove_token(&mut self, r: TokenRef) -> Option<Token> {
        Some(self.remove_token_at(self.token_position(r)?))
    }

    pub(crate) fn remove_token_at(&mut self, pos: usize) -> Token {
        self.inner_tokens.remove(pos).1
    }

    pub(crate) fn remove_whitespace_before(&mut self, before_this: TokenRef) {
        let Some(left) = self.token_position(before_this).unwrap().checked_sub(1) else {
            return;
        };

        if self.get_token_at(left).is_some_and(|t| t.is_whitespace()) {
            self.remove_token_at(left);
        }
    }

    pub(crate) fn remove_range(&mut self, left: TokenRef, right: TokenRef) {
        let left = self.token_position(left).unwrap();
        let diff = self.token_position(right).unwrap() - left;

        for _ in 0..=diff {
            self.remove_token_at(left);
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
