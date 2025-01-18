use itertools::Itertools;

use super::{DocumentRef, SealedObjectLike, Structure};
use crate::Document;

#[derive(Debug, Clone)]
pub struct RootObject(Vec<(DocumentRef, Structure)>);

impl SealedObjectLike for RootObject {
    fn raw_kvs(&self) -> &Vec<(DocumentRef, Structure)> { &self.0 }
    fn raw_kvs_mut(&mut self) -> &mut Vec<(DocumentRef, Structure)> { &mut self.0 }
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
