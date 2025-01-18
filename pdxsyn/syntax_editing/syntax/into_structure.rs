use crate::{Document, IntoLiteral, Literal};

use super::{DocumentRef, Structure};

pub(crate) trait IntoStructure {
    fn into_structure(&self, doc: &mut Document, insert_after: DocumentRef) -> Structure;
}

impl<T: IntoLiteral> IntoStructure for T {
    fn into_structure(&self, doc: &mut Document, insert_after: DocumentRef) -> Structure {
        todo!()
    }
}

impl IntoStructure for &[Box<dyn IntoLiteral>] {
    fn into_structure(&self, doc: &mut Document, insert_after: DocumentRef) -> Structure {
        todo!()
    }
}

impl IntoStructure for &[(Literal, Box<dyn IntoStructure>)] {
    fn into_structure(&self, doc: &mut Document, insert_after: DocumentRef) -> Structure {
        todo!()
    }
}
