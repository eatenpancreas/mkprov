use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub struct Color([u8; 3]);

impl Color {
    pub fn new(rgb: [u8; 3]) -> Self { Self(rgb) }
}
