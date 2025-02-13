use std::fmt::{Debug, Display};

use clap::ValueEnum;
use thiserror::Error;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum ItemKind {
    /// Aliases = p, prov
    #[clap(aliases(["p", "prov"]))]
    Province,
    /// Aliases = a
    #[clap(alias("a"))]
    Area,
    /// Aliases = c
    #[clap(alias("c"))]
    Country,
}

#[derive(Clone, Copy)]
pub struct CountryTag([char; 3]);

impl Debug for CountryTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "\"{self}\"") }
}

impl Display for CountryTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

#[derive(Error, Debug)]
#[error("Country tag expects exactly 3 letters")]
pub struct CountryTagError;

impl CountryTag {
    pub fn new(tag: impl IntoIterator<Item = char>) -> Result<Self, CountryTagError> {
        let mut it = tag.into_iter();

        let mut get_ch = || {
            let ch = it.next()?;
            let ch = ch.to_ascii_uppercase();
            ch.is_ascii_uppercase().then_some(ch)
        };

        let tag = CountryTag([
            get_ch().ok_or(CountryTagError)?,
            get_ch().ok_or(CountryTagError)?,
            get_ch().ok_or(CountryTagError)?,
        ]);

        it.next().is_none().then_some(tag).ok_or(CountryTagError)
    }
}
