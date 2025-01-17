pub mod parsing;
pub mod syntax;

use itertools::Itertools;
use slotmap::{new_key_type, SlotMap};

use crate::{Literal, Token};

new_key_type! {
    pub(crate) struct DocumentRef;
}

#[derive(Clone, Debug)]
pub struct Document {
    inner_tokens: SlotMap<DocumentRef, Token>,
    inner_ordered_keys: Vec<DocumentRef>,
}

impl Document {
    pub fn create(tokens: Vec<Token>) -> Self {
        let mut mapped_tokens = SlotMap::with_key();
        let ordered_keys = tokens
            .into_iter()
            .map(|token| mapped_tokens.insert(token))
            .collect();

        Self {
            inner_tokens: mapped_tokens,
            inner_ordered_keys: ordered_keys,
        }
    }

    pub(crate) fn get_literal(&self, r: DocumentRef) -> Option<&Literal> {
        self.inner_ordered_keys.first()?;
        self.inner_tokens.get(r)?.as_literal()
    }

    pub(crate) fn get_literal_mut(&mut self, r: DocumentRef) -> Option<&mut Literal> {
        self.inner_ordered_keys.first()?;
        self.inner_tokens.get_mut(r)?.as_literal_mut()
    }

    pub(crate) fn get_token(&self, r: DocumentRef) -> Option<&Token> {
        self.inner_ordered_keys.first()?;
        self.inner_tokens.get(r)
    }

    pub(crate) fn ref_pos(&self, r: DocumentRef) -> Option<usize> {
        self.inner_ordered_keys.iter().position(|t| *t == r)
    }

    pub(crate) fn get_pos(&self, pos: usize) -> Option<&Token> {
        let key = *self.inner_ordered_keys.get(pos)?;
        self.inner_tokens.get(key)
    }

    pub(crate) fn remove_pos(&mut self, pos: usize) -> Option<Token> {
        let key = self.inner_ordered_keys.remove(pos);
        self.inner_tokens.remove(key)
    }

    pub(crate) fn remove_whitespace_before(&mut self, before_this: DocumentRef) {
        let Some(left) = self.ref_pos(before_this).unwrap().checked_sub(1) else {
            return;
        };

        if self.get_pos(left).is_some_and(|t| t.is_whitespace()) {
            self.remove_pos(left);
        }
    }

    pub(crate) fn remove_range(&mut self, left: DocumentRef, right: DocumentRef) {
        let left = self.ref_pos(left).unwrap();
        let diff = self.ref_pos(right).unwrap() - left;

        for _ in 0..=diff {
            self.remove_pos(left);
        }
    }

    pub fn into_string(self) -> String {
        self.return_tokens().iter().map(|t| t.to_string()).join("")
    }

    pub fn return_tokens(mut self) -> Vec<Token> {
        self.inner_ordered_keys
            .into_iter()
            .filter_map(|r| self.inner_tokens.remove(r))
            .collect()
    }

    pub(crate) fn ref_position(&self, dr: DocumentRef) -> usize {
        let mut total_len = 0;
        for token_ref in self.inner_ordered_keys.iter() {
            if *token_ref == dr {
                break;
            }
            total_len += self.get_token(*token_ref).unwrap().to_string().len();
        }

        total_len
    }
}
