use itertools::Itertools;

use super::{TokenRef, SealedObjectLike, Structure};
use crate::Document;

#[derive(Debug, Clone)]
pub struct RootObject(Vec<(TokenRef, Structure)>);

impl SealedObjectLike for RootObject {
    fn raw_kvs(&self) -> &Vec<(TokenRef, Structure)> { &self.0 }
    fn raw_kvs_mut(&mut self) -> &mut Vec<(TokenRef, Structure)> { &mut self.0 }
}

impl RootObject {
    pub(crate) fn new() -> Self { Self(vec![]) }

    pub fn debug_fmt(&self, doc: &Document) -> String {
        self.raw_kvs()
            .iter()
            .format_with("\n", |(d_ref, s), f| {
                f(&format_args!(
                    "{} = {}",
                    doc.get_token(*d_ref).unwrap(),
                    s.debug_fmt_inner(doc, 0)
                ))
            })
            .to_string()
    }
}
