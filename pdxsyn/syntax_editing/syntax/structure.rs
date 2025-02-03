use super::{Array, Object, SealedSyntaxLike, TokenRef, Value};
use crate::Document;

#[derive(Debug, Clone)]
pub enum Structure {
    Object(Object),
    Value(Value),
    Array(Array),
}

impl SealedSyntaxLike for Structure {
    fn token_range(&self) -> (TokenRef, Option<TokenRef>) {
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
    pub fn as_value(&self) -> Option<&Value> {
        match self {
            Self::Value(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_value_mut(&mut self) -> Option<&mut Value> {
        match self {
            Self::Value(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Self::Array(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        match self {
            Self::Array(s) => Some(s),
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

    pub fn debug_fmt(&self, doc: &Document) -> String {
        match self {
            Self::Value(o) => o.debug_fmt(doc),
            Self::Object(o) => o.debug_fmt(doc),
            Self::Array(o) => o.debug_fmt(doc),
        }
    }
}
