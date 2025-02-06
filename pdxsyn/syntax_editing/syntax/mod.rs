mod array;
mod into_structure;
mod object;
mod object_like;
mod root_object;
mod structure;
mod value;

use super::{Document, TokenRef};
pub use object_like::ObjectLike;
pub use {array::*, into_structure::*, object::*, root_object::*, structure::*, value::*};

pub(crate) trait SealedSyntaxLike {
    fn token_range(&self) -> (TokenRef, Option<TokenRef>);
}
impl<T> SyntaxLike for T where T: SealedSyntaxLike {}

#[allow(private_bounds)]
pub trait SyntaxLike: SealedSyntaxLike {}

pub(crate) trait SealedObjectLike {
    fn raw_kvs(&self) -> &Vec<(TokenRef, Structure)>;
    fn raw_kvs_mut(&mut self) -> &mut Vec<(TokenRef, Structure)>;
    fn indentation(&self) -> usize;
}
impl<T> ObjectLike for T where T: SealedObjectLike {}

pub trait DebugFmt {
    fn debug_fmt(&self, doc: &Document) -> String;
}
