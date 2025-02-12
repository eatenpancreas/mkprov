pub mod common;
pub mod query;
pub mod structure;

use clap::{Parser, Subcommand};
use common::ItemKind;
use mkprov_lib::workspace::Workspace;
use query::QueryArgs;
use structure::ProvinceCsv;

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(version, about)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

pub fn main(workspace: Workspace) {
    let args = Args::parse();
    match args.command {
        Command::Query(args) => match args.kind {
            ItemKind::Province => {
                for item in args.items {
                    let csv = ProvinceCsv::load(&workspace);
                    let id = csv.search_name(&item);
                    println!("{id:?}");
                }
            }
            _ => {}
        },
        Command::Make(_args) => {}
    };
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
    /// Aliases = q
    #[clap(aliases(["q"]))]
    Query(QueryArgs),
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
