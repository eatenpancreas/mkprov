mod common;

mod commands {
    pub mod cfg;
    pub mod own;
    pub mod cp;
    pub mod cp_area;
    pub mod mk;
    pub mod rn;
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
    /// Makes a list of non-owned, basic provinces in MOD/history/provinces. makes definitions, etc
    Mk(cmd::mk::CmdArgs),
    /// Converts a province to an owner tag.
    Own(cmd::own::CmdArgs),
    /// Changes project configs.
    Cfg(cmd::cfg::CmdArgs),
    /// Copies a province defines to another
    Cp(cmd::cp::CmdArgs),
    /// Gets the area from one province and puts it in the other
    CpArea(cmd::cp_area::CmdArgs),
    /// Renames a province
    Rn(cmd::rn::CmdArgs),
}

fn main() {
    match Args::parse().command {
        Command::Mk(args) => cmd::mk::run(args),
        Command::Own(args) => cmd::own::run(args),
        Command::Cfg(args) => cmd::cfg::run(args),
        Command::Cp(args) => cmd::cp::run(args),
        Command::CpArea(args) => cmd::cp_area::run(args),
        Command::Rn(args) => cmd::rn::run(args),
    };
}
