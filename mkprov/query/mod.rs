use clap::{Parser, Subcommand, ValueEnum};
/// The query part of Mkprov
#[derive(Parser, Debug)]
pub struct QueryArgs {
    /// The kind of item selected
    kind: ItemKind,
    /// The item names or ids
    items: Vec<String>,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum ItemKind {
    /// Select one or more provinces
    Prov,
    /// Select one or more areas
    Area,
}
