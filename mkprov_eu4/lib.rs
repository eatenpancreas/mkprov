pub mod common;
pub mod query;
pub mod structure;

use clap::{Parser, Subcommand};
use common::ItemKind;
use query::{QueryActions, QueryArgs};

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    /// Aliases = q
    #[clap(aliases(["q"]))]
    Query {
        #[clap(flatten)]
        query: QueryArgs,
        #[clap(flatten)]
        actions: QueryActions,
    },
    /// Aliases = mk, new
    #[clap(aliases(["mk", "new"]))]
    Make(MakeArgs),
}

#[derive(clap::Args, Clone, Debug)]
pub struct MakeArgs {
    /// The kind of item created
    kind: ItemKind,
    /// The name of the item created
    name: String,
}
