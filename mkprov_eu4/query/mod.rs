use crate::common::ItemKind::{self};

#[derive(clap::Args, Clone, Debug)]
pub struct QueryArgs {
    /// The kind of items selected
    pub(crate) kind: ItemKind,
    /// The item names or ids
    pub(crate) items: Vec<String>,
    /// Sets the continent of the query
    #[clap(flatten)]
    pub(crate) actions: QueryActions,
}

/// Actions
#[derive(clap::Args, Clone, Debug)]
pub struct QueryActions {
    /// [action] Sets the owner of the queried items
    #[arg(short, long)]
    pub(crate) owner: Option<String>,
    /// [action] Sets the continent of the queried items
    #[arg(long)]
    pub(crate) continent: Option<String>,
    /// [action] Sets the tradenode of the queried items
    #[arg(long)]
    pub(crate) tradenode: Option<String>,
    /// [action] Sets the climate of the queried items
    #[arg(long)]
    pub(crate) climate: Option<ClimateType>,
    /// [action] Sets the monsoon of the queried items
    #[arg(long)]
    pub(crate) monsoon: Option<Severity>,
    /// [action] Sets the winter of the queried items
    #[arg(long)]
    pub(crate) winter: Option<Severity>,
    /// [action] Deletes the queried items
    #[arg(long)]
    pub(crate) delete: bool,
    /// [action] Prints the ids of the provinces in the queried items
    #[arg(long)]
    pub(crate) print: bool,
}

use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum ClimateType {
    Tropical,
    Arid,
    Arctic,
    #[default]
    Normal,
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum Severity {
    Normal,
    Mild,
    Severe,
    #[default]
    None,
}
