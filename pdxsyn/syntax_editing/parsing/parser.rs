use super::{ParseDocumentError, TokenRef};
use crate::{Document, Token};

pub(crate) use ExpectAction::*;

#[derive(Clone)]
pub(crate) struct DocumentParser<'a> {
    document: &'a Document,
    cursor: usize,
}

#[derive(Clone, Copy)]
pub(crate) enum ExpectAction<T> {
    Valid(T),
    Ignore,
    Invalid,
    End,
}

impl<'a> DocumentParser<'a> {
    pub(crate) fn new(document: &'a Document) -> Self { Self { document, cursor: 0 } }
    pub(crate) fn document(&self) -> &Document { self.document }

    pub(crate) fn pop(&mut self) -> Option<(TokenRef, &Token)> {
        self.advance();
        self.peek_nth(self.cursor - 1)
    }

    fn peek_nth(&self, pos: usize) -> Option<(TokenRef, &Token)> {
        self.document()
            .inner_tokens
            .get(pos)
            .map(|(dr, token)| (*dr, token))
    }

    fn peek(&self) -> Option<(TokenRef, &Token)> { self.peek_nth(self.cursor + 1) }
    fn advance(&mut self) { self.cursor += 1 }

    pub(crate) fn peek_until_expected<T>(
        &mut self,
        expecter: impl Fn(&Token) -> ExpectAction<T>,
        expect_str: &'static str,
    ) -> Result<Option<(TokenRef, T)>, ParseDocumentError> {
        while let Some((r, t)) = self.peek() {
            match (expecter)(t) {
                Valid(t) => return Ok(Some((r, t))),
                Ignore => continue,
                End => return Ok(None),
                Invalid => {
                    return Err(ParseDocumentError::UnexpectedToken(
                        t.to_string(),
                        self.document().token_position(r).unwrap_or(0),
                        expect_str,
                    ));
                }
            }
        }

        Ok(None)
    }

    pub(crate) fn pop_until_expected<T>(
        &mut self,
        expecter: impl Fn(&Token) -> ExpectAction<T>,
        expect_str: &'static str,
    ) -> Result<Option<(TokenRef, T)>, ParseDocumentError> {
        while let Some((r, t)) = self.pop() {
            match (expecter)(t) {
                Valid(t) => return Ok(Some((r, t))),
                Ignore => continue,
                End => return Ok(None),
                Invalid => {
                    return Err(ParseDocumentError::UnexpectedToken(
                        t.to_string(),
                        self.document().token_position(r).unwrap_or(0),
                        expect_str,
                    ));
                }
            }
        }

        Ok(None)
    }
}
