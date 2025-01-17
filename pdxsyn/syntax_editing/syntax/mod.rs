mod array;
mod object;
mod root_object;
mod structure;
mod value;

pub use {array::*, object::*, root_object::*, structure::*, value::*};

use crate::{IntoLiteral, Literal};

use super::{Document, DocumentRef};

pub(crate) trait SealedSyntaxLike {
    fn token_range(&self) -> (DocumentRef, Option<DocumentRef>);
}
impl<T> SyntaxLike for T where T: SealedSyntaxLike {}

#[allow(private_bounds)]
pub trait SyntaxLike: SealedSyntaxLike {}

pub(crate) trait SealedObjectLike {
    fn raw_kvs(&self) -> &Vec<(DocumentRef, Structure)>;
    fn raw_kvs_mut(&mut self) -> &mut Vec<(DocumentRef, Structure)>;
}
impl<T> ObjectLike for T where T: SealedObjectLike {}

#[allow(private_bounds)]
pub trait ObjectLike: SealedObjectLike {
    /// removes a single instance of the key. Returns true if there's more
    fn remove_key<'a, T: IntoLiteral>(&'a mut self, doc: &'a mut Document, key: T) -> Option<()> {
        let key = key.into_literal();

        let kvs = self.raw_kvs_mut();

        let Some(idx) = kvs
            .iter()
            .position(|(r, _)| doc.get_literal(*r) == Some(&key))
        else {
            return None;
        };

        let (d_ref, s) = kvs.remove(idx);
        let (middle, right) = s.token_range();
        doc.remove_whitespace_before(d_ref);
        doc.remove_range(d_ref, right.unwrap_or(middle));

        Some(())
    }

    fn iter_keys<'a>(&'a self, doc: &'a Document) -> impl Iterator<Item = &'a Literal> {
        self.raw_kvs()
            .iter()
            .map(|(r, _)| doc.get_literal(*r).unwrap())
    }

    fn iter_key_values_mut<'a, 'b>(
        &'a mut self,
        doc: &'b Document,
    ) -> impl Iterator<Item = (&'b Literal, &'a mut Structure)> {
        self.raw_kvs_mut()
            .iter_mut()
            .map(move |(r, s)| (doc.get_literal(*r).unwrap(), s))
    }

    fn iter_key_values<'a, 'b>(
        &'a self,
        doc: &'b Document,
    ) -> impl Iterator<Item = (&'b Literal, &'a Structure)> {
        self.raw_kvs()
            .iter()
            .map(|(r, s)| (doc.get_literal(*r).unwrap(), s))
    }

    fn iter_key_indices<'a, T: IntoLiteral>(
        &'a self,
        doc: &'a Document,
        key: T,
    ) -> impl Iterator<Item = usize> {
        let key = key.into_literal();
        self.iter_keys(doc)
            .enumerate()
            .filter_map(move |(i, lit)| (lit == &key).then_some(i))
    }

    fn get_key_at<'a, 'b>(&'a self, doc: &'b Document, index: usize) -> Option<&'b Literal> {
        self.raw_kvs()
            .get(index)
            .map(move |(r, _)| doc.get_literal(*r).unwrap())
    }

    fn get_value_at(&self, index: usize) -> Option<&Structure> {
        self.raw_kvs().get(index).map(|(_, s)| s)
    }

    fn get_value_at_mut(&mut self, index: usize) -> Option<&mut Structure> {
        self.raw_kvs_mut().get_mut(index).map(|(_, s)| s)
    }

    /// Gets the first occurrence of the key and returns the mutable value. Returns None if the key doesn't occur
    fn get_first_mut<'a, 'b, T: IntoLiteral>(
        &'a mut self,
        doc: &'b mut Document,
        key: T,
    ) -> Option<&'a mut Structure> {
        let idx = self.iter_key_indices(doc, key).next()?;
        self.get_value_at_mut(idx)
    }

    /// Gets the first occurrence of the key and returns the value. Returns None if the key doesn't occur
    fn get_first<'a, T: IntoLiteral>(&'a self, doc: &'a Document, key: T) -> Option<&'a Structure> {
        let idx = self.iter_key_indices(doc, key).next()?;
        self.get_value_at(idx)
    }
}
