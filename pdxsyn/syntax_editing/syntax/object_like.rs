use super::{IntoStructure, SealedObjectLike, SealedSyntaxLike, Structure};
use crate::{Document, IntoLiteral, Literal, Token};

#[allow(private_bounds)]
pub trait ObjectLike: SealedObjectLike {
    /// Removes the first occurrence of the specified key from the object.
    /// Returns false if the key did not exist in the object
    fn remove_key<T: IntoLiteral>(&mut self, doc: &mut Document, key: T) -> bool {
        let key = key.into_literal();

        let Some(idx) = self.iter_key_indices(doc, key).next() else {
            return false;
        };

        self.remove_index(doc, idx).is_some()
    }

    /// Removes the key-value at the given index and returns the key
    fn remove_index(&mut self, doc: &mut Document, index: usize) -> Option<Literal> {
        let kvs = self.raw_kvs_mut();
        if index >= kvs.len() {
            return None;
        }
        let (d_ref, s) = kvs.remove(index);
        let (middle, right) = s.token_range();

        if let Some(left) = doc.token_sub_position(d_ref, 1) {
            if doc.get_token(left).is_some_and(|t| t.is_whitespace()) {
                doc.remove_token(left);
            }
        }

        doc.remove_range(d_ref, right.unwrap_or(middle))
            .into_iter()
            .next()
            .map(|t| t.into_literal().unwrap())
    }

    fn has_key<T: PartialEq<Literal>>(&self, doc: &Document, key: T) -> bool {
        self.iter_keys(doc).any(|k| &key == k)
    }

    /// Checks if `self` has the same keys as `other`, making it the same kind of object
    fn has_same_keys_as(&self, doc: &Document, other: &impl ObjectLike) -> bool {
        self.iter_keys(doc)
            .enumerate()
            .all(|(i, lit)| other.iter_keys(doc).nth(i) == Some(lit))
    }

    /// Iterates over the keys of the object
    fn iter_keys<'a>(&self, doc: &'a Document) -> impl Iterator<Item = &'a Literal> {
        self.raw_kvs()
            .iter()
            .map(|(r, _)| doc.get_literal(*r).expect("Expected literal"))
    }

    /// Iterates over the key-value pairs of the object, allowing mutable access to the values.
    fn iter_key_values_mut<'a>(
        &mut self,
        doc: &'a Document,
    ) -> impl Iterator<Item = (&'a Literal, &mut Structure)> {
        self.raw_kvs_mut()
            .iter_mut()
            .map(move |(r, s)| (doc.get_literal(*r).expect("Expected literal"), s))
    }

    /// Iterates over the key-value pairs of the object
    fn iter_key_values<'a>(
        &self,
        doc: &'a Document,
    ) -> impl Iterator<Item = (&'a Literal, &Structure)> {
        self.raw_kvs()
            .iter()
            .map(|(r, s)| (doc.get_literal(*r).expect("Expected literal"), s))
    }

    /// Iterates over the key-value pairs in the object, returning an iterator
    /// where each item is a index of the object keys.
    fn iter_key_indices<T: IntoLiteral>(
        &self,
        doc: &Document,
        key: T,
    ) -> impl Iterator<Item = usize> {
        let key = key.into_literal();
        self.iter_keys(doc)
            .enumerate()
            .filter_map(move |(i, lit)| (lit == &key).then_some(i))
    }

    fn insert<'a>(
        &mut self,
        doc: &'a mut Document,
        index: usize,
        key: impl IntoLiteral,
        value: impl IntoStructure,
    ) {
        let key = Token::Literal(key.into_literal());

        if let Some((k, _)) = self.raw_kvs().get(index).or_else(|| self.raw_kvs().last()) {
            let indent = "   ".repeat(self.indentation());

            let inserted = doc.insert_tokens_before(
                vec![
                    key,
                    Token::Whitespace(" ".to_owned()),
                    Token::Equals,
                    Token::Whitespace(" ".to_owned()),
                ],
                *k,
            );

            let struc = Box::new(value).into_structure(
                doc,
                *inserted.last().unwrap(),
                self.indentation() + 1,
            );

            let (start, end) = struc.token_range();
            doc.insert_token_after(Token::Whitespace(format!("\n{indent}")), end.unwrap_or(start));

            self.raw_kvs_mut().insert(index, (inserted[0], struc));
        };
    }

    /// Gets the key at the specified index and returns a reference to it.
    /// Returns None if the index is out of bounds.
    fn get_key_at<'a>(&self, doc: &'a Document, index: usize) -> Option<&'a Literal> {
        self.raw_kvs()
            .get(index)
            .map(move |(r, _)| doc.get_literal(*r).unwrap())
    }

    /// Gets the value at the specified index and returns a reference to it.
    /// Returns None if the index is out of bounds.
    fn get_value_at(&self, index: usize) -> Option<&Structure> {
        self.raw_kvs().get(index).map(|(_, s)| s)
    }

    /// Gets the value at the specified index and returns a mutable reference to it.
    /// Returns None if the index is out of bounds.
    fn get_value_at_mut(&mut self, index: usize) -> Option<&mut Structure> {
        self.raw_kvs_mut().get_mut(index).map(|(_, s)| s)
    }

    /// Gets the value at the specified key and returns a mutable reference to it.
    /// Returns None if the the key doesn't occur
    fn get_first_mut<T: IntoLiteral>(
        &mut self,
        doc: &mut Document,
        key: T,
    ) -> Option<&mut Structure> {
        let idx = self.iter_key_indices(doc, key).next()?;
        self.get_value_at_mut(idx)
    }

    /// Gets the value at the specified key and returns a reference to it.
    /// Returns None if the the key doesn't occur
    fn get_first<T: IntoLiteral>(&self, doc: &Document, key: T) -> Option<&Structure> {
        let idx = self.iter_key_indices(doc, key).next()?;
        self.get_value_at(idx)
    }
}
