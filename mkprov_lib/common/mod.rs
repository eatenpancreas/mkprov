use std::fmt::Display;
use std::io;

use crossterm::{
    execute,
    style::{Print, PrintStyledContent, Stylize},
};
use derived_deref::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct Color([u8; 3]);

impl Color {
    pub fn new(rgb: [u8; 3]) -> Self { Self(rgb) }
}

pub fn print_error(message: impl Display) {
    let err = "error: ".to_string().bold().dark_red();
    let mut stdout = io::stdout();
    execute!(stdout, PrintStyledContent(err), Print(message), Print('\n')).unwrap();
}
