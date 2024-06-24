mod common;

mod commands {
    pub mod config;
    pub mod definition_list;
    pub mod list;
    pub mod own;
    pub mod safe_make_provs;
}

use crate::commands::{config, definition_list, list, own, safe_make_provs};
use clap::{Parser, Subcommand};

/// Simple program to make a list of provinces
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Makes a list of non-owned, basic provinces in MOD/history/provinces.
    SafeMakeProvs(safe_make_provs::CmdArgs),
    /// Makes a simple list province id's for use in area.txt, default.map, etc.
    List(list::CmdArgs),
    /// Makes a list of provinces for use in definition.csv, picking random colors and iterating through them.
    DefinitionList(definition_list::CmdArgs),
    /// Converts a province to an owner tag.
    Own(own::CmdArgs),
    /// Changes project configs.
    Config(config::CmdArgs),
}

fn main() {
    match Args::parse().command {
        Command::SafeMakeProvs(args) => safe_make_provs::run(args),
        Command::List(args) => list::run(args),
        Command::DefinitionList(args) => definition_list::run(args),
        Command::Own(args) => own::run(args),
        Command::Config(args) => config::run(args),
    };
}
