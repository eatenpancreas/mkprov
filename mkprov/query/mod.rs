use ItemKind::*;
use clap::{Parser, ValueEnum};
use mod_workspace::Workspace;

/// The query part of Mkprov.
/// Use mkprov --action to display help for the action part of Mkprov
///
/// Combine the output with the action part using the pipe operator |
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(name = "mkprov (query)")]
#[command(bin_name = "mkprov")]
#[command(version, about)]
pub struct QueryArgs {
    /// The kind of item selected
    #[arg(required_unless_present = "action")]
    pub kind: Option<ItemKind>,
    /// The item names or ids
    pub items: Vec<String>,
    #[arg(long, help = "Show help for the second part of mkprov")]
    pub action: bool,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum ItemKind {
    /// Select one or more provinces.
    /// Aliases = p, prov
    #[clap(aliases(["p", "prov"]))]
    Province,
    /// Select one or more areas.
    /// Aliases = a
    #[clap(alias("a"))]
    Area,
}

impl QueryArgs {
    pub fn main(kind: ItemKind, _items: Vec<String>, _workspace: Workspace) {
        match kind {
            Province => {}
            Area => {}
        }
    }
}
