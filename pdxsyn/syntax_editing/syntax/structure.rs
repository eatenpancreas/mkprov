use super::{Array, DocumentRef, Object, SealedSyntaxLike, Value};
use crate::Document;

#[derive(Debug, Clone)]
pub enum Structure {
    Object(Object),
    Value(Value),
    Array(Array),
}

impl SealedSyntaxLike for Structure {
    fn token_range(&self) -> (DocumentRef, Option<DocumentRef>) {
        match self {
            Self::Array(a) => a.token_range(),
            Self::Value(a) => a.token_range(),
            Self::Object(a) => a.token_range(),
        }
    }
}

impl Structure {
    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Self::Object(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            Self::Object(s) => Some(s),
            _ => None,
        }
    }
    pub fn is_object(&self) -> bool {
        match self {
            Self::Object(_) => true,
            _ => false,
        }
    }
    pub fn is_value(&self) -> bool {
        match self {
            Self::Value(_) => true,
            _ => false,
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_) => true,
            _ => false,
        }
    }

    pub(crate) fn debug_fmt_inner(&self, doc: &Document, nesting: usize) -> String {
        match self {
            Self::Value(o) => o.debug_fmt(doc),
            Self::Object(o) => o.debug_fmt_inner(doc, nesting + 1),
            Self::Array(o) => o.debug_fmt_inner(doc, nesting + 1),
        }
    }

    pub fn debug_fmt(&self, doc: &Document) -> String {
        self.debug_fmt_inner(doc, 0)
    }
}
