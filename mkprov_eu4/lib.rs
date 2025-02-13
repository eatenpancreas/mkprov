pub mod common;
pub mod query;
pub mod structure;

use clap::{CommandFactory, Parser, Subcommand};
use common::{CountryTag, ItemKind};
use itertools::Itertools;
use mkprov_lib::{common::print_error, workspace::Workspace};
use query::{QueryActions, QueryArgs};
use structure::{ProvinceCsv, ProvinceHistories};

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
                if args.items.is_empty() {
                    print_error("Failed to provide items for mkprov query.");
                    Args::command()
                        .find_subcommand_mut("query")
                        .unwrap()
                        .print_help()
                        .unwrap();
                }

                let ids = args
                    .items
                    .into_iter()
                    .filter_map(|item| {
                        let csv = ProvinceCsv::load(&workspace);
                        csv.search_name(&item)
                    })
                    .collect_vec();

                match args.actions {
                    QueryActions {
                        owner: None,
                        continent: None,
                        tradenode: None,
                        climate: None,
                        monsoon: None,
                        winter: None,
                        delete: false,
                        print: false,
                    } => {
                        print_error("No action given for mkprov query.");

                        Args::command()
                            .find_subcommand_mut("query")
                            .unwrap()
                            .print_help()
                            .unwrap();
                    }
                    _ => {
                        if let Some(owner) = args.actions.owner {
                            for id in ids {
                                match ProvinceHistories::load(&workspace).find_id(id, &workspace) {
                                    Some(mut hist) => {
                                        hist.set_owner(CountryTag::new(owner.chars()).unwrap());
                                        hist.save(&workspace).unwrap();
                                    }
                                    None => println!("could not find {id} in province histories!"),
                                }
                            }
                        }
                    }
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
