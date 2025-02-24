use derived_deref::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct Color([u8; 3]);

impl Color {
    pub fn new(rgb: [u8; 3]) -> Self { Self(rgb) }
}
