mod common;
mod prov_commands;
mod country_commands;

mod base_commands {
    pub mod cfg;
}

use clap::{Parser, Subcommand};
use paradox_file::Config;

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
    // - group commands
    Prov(prov_commands::GroupArgs),
    Country(country_commands::GroupArgs),
    
    // - base commands
    /// Changes project configs
    Cfg(base_commands::cfg::CmdArgs),
}

fn main() {
    let config = Config::current();
    
    
    match Args::parse().command {
        Command::Prov(args) => args.run(),
        Command::Country(args) => args.run(),
        
        Command::Cfg(args) => args.run(),
    };
}
