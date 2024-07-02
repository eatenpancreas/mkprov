mod common;

mod commands {
    pub mod config;
    pub mod definition_list;
    pub mod list;
    pub mod own;
    pub mod copy;
    pub mod area;
    pub mod safe_make_provs;
    pub mod rename;
}

use clap::{Parser, Subcommand};
use commands as cmd;

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
    SafeMakeProvs(cmd::safe_make_provs::CmdArgs),
    /// Makes a simple list province id's for use in area.txt, default.map, etc.
    List(cmd::list::CmdArgs),
    /// Makes a list of provinces for use in definition.csv, picking random colors and iterating through them.
    DefinitionList(cmd::definition_list::CmdArgs),
    /// Converts a province to an owner tag.
    Own(cmd::own::CmdArgs),
    /// Changes project configs.
    Config(cmd::config::CmdArgs),
    /// Copies a province defines to another
    Copy(cmd::copy::CmdArgs),
    /// Gets the area from one province and puts it in the other
    Area(cmd::area::CmdArgs),
    /// Renames a province
    Rename(cmd::rename::CmdArgs),
}

fn main() {
    match Args::parse().command {
        Command::SafeMakeProvs(args) => cmd::safe_make_provs::run(args),
        Command::List(args) => cmd::list::run(args),
        Command::DefinitionList(args) => cmd::definition_list::run(args),
        Command::Own(args) => cmd::own::run(args),
        Command::Config(args) => cmd::config::run(args),
        Command::Copy(args) => cmd::copy::run(args),
        Command::Area(args) => cmd::area::run(args),
        Command::Rename(args) => cmd::rename::run(args),
    };
}
