use super::{DocumentRef, ParseDocumentError};
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
    pub(crate) fn new(document: &'a Document) -> Self {
        Self { document, cursor: 0 }
    }

    pub(crate) fn document(&self) -> &Document {
        self.document
    }

    pub(crate) fn pop(&mut self) -> Option<(DocumentRef, &Token)> {
        self.advance();
        self.peek_nth(self.cursor - 1)
    }

    fn peek_nth(&self, pos: usize) -> Option<(DocumentRef, &Token)> {
        let key = self.document.inner_ordered_keys.get(pos)?;
        Some((*key, &self.document.inner_tokens[*key]))
    }

    fn peek(&self) -> Option<(DocumentRef, &Token)> {
        self.peek_nth(self.cursor + 1)
    }

    fn advance(&mut self) {
        self.cursor += 1
    }

    pub(crate) fn peek_until_expected<T>(
        &mut self,
        expecter: impl Fn(&Token) -> ExpectAction<T>,
        expect_str: &'static str,
    ) -> Result<Option<(DocumentRef, T)>, ParseDocumentError> {
        while let Some((r, t)) = self.peek() {
            match (expecter)(t) {
                Valid(t) => return Ok(Some((r, t))),
                Ignore => continue,
                End => return Ok(None),
                Invalid => {
                    return Err(ParseDocumentError::UnexpectedToken(
                        t.to_string(),
                        self.document().ref_position(r),
                        expect_str,
                    ))
                }
            }
        }

        Ok(None)
    }

    pub(crate) fn pop_until_expected<T>(
        &mut self,
        expecter: impl Fn(&Token) -> ExpectAction<T>,
        expect_str: &'static str,
    ) -> Result<Option<(DocumentRef, T)>, ParseDocumentError> {
        while let Some((r, t)) = self.pop() {
            match (expecter)(t) {
                Valid(t) => return Ok(Some((r, t))),
                Ignore => continue,
                End => return Ok(None),
                Invalid => {
                    return Err(ParseDocumentError::UnexpectedToken(
                        t.to_string(),
                        self.document().ref_position(r),
                        expect_str,
                    ))
                }
            }
        }

        Ok(None)
    }
}
