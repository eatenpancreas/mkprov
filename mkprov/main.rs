mod common;

pub mod commands {
    pub mod cfg;
    pub mod own;
    pub mod cp;
    pub mod mv_area;
    pub mod mk;
    pub mod rn;
    pub mod convert;
}

use clap::{Parser, Subcommand};
use commands as cmd;
use Command::*;

/// CLI-interface to create and edit Paradox files.
/// To start, run mkprov(.exe) cfg to edit the config.
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
    /// Converts a province to an owner tag
    Own(cmd::own::CmdArgs),
    /// Changes project configs
    Cfg(cmd::cfg::CmdArgs),
    /// Copies a province history to another.
    /// Optionally also uses cp-area to copy both
    Cp(cmd::cp::CmdArgs),
    /// Moves a province from an area to another
    MvArea(cmd::mv_area::CmdArgs),
    /// Renames a province
    Rn(cmd::rn::CmdArgs),
    /// Converts a province to a religion or culture
    Convert(cmd::convert::CmdArgs)
}

fn main() {
    match Args::parse().command {
        Mk(args) => cmd::mk::run(args),
        Own(args) => cmd::own::run(args),
        Cfg(args) => cmd::cfg::run(args),
        Cp(args) => cmd::cp::run(args),
        MvArea(args) => cmd::mv_area::run(args),
        Rn(args) => cmd::rn::run(args),
        Convert(args) => cmd::convert::run(args),
    };
}
