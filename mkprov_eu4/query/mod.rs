mod action;
use crate::common::ItemKind::{self};
pub use action::*;

#[derive(clap::Args, Clone, Debug)]
pub struct QueryArgs {
    /// The kind of items selected
    pub(crate) kind: ItemKind,
    /// The item names or ids
    pub(crate) items: Vec<String>,
    /// Sets the continent of the query
    #[clap(flatten)]
    actions: QueryActions,
}

/// Actions
#[derive(clap::Args, Clone, Debug)]
pub struct QueryActions {
    /// Sets the owner of the queried items
    #[arg(short, long)]
    owner: Option<String>,
    /// Sets the continent of the queried items
    #[arg(long)]
    continent: Option<String>,
    /// Sets the tradenode of the queried items
    #[arg(long)]
    tradenode: Option<String>,
    /// Sets the climate of the queried items
    #[arg(long)]
    climate: Option<ClimateType>,
    /// Sets the monsoon of the queried items
    #[arg(long)]
    monsoon: Option<Severity>,
    /// Sets the winter of the queried items
    #[arg(long)]
    winter: Option<Severity>,
    /// Deletes the queried items
    #[arg(long)]
    delete: bool,
}
