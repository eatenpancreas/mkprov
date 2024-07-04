mod common;
mod prov_commands;
mod country_commands;

mod base_commands {
    pub mod cfg;
}

use clap::{Parser, Subcommand};
use inquire::Confirm;
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
    let mut config = Config::current().unwrap();
    if config.is_first_time {
        let ans = Confirm::new(
            "mkprov is a tool designed for total overhaul mods for eu4.\
            There are a couple issues, particularly it'll modify files in the mod for convenience.\
            There is currently no support for comments, and as it modifies files it will gradually \
            delete all comments in files it touches. Make sure you are okay with this before using \
            the tool."
        )
          .with_default(false)
          .with_help_message("Are you okay with this?")
          .prompt().unwrap_or(false);
        
        if !ans { return; }
        config.is_first_time = false;
        config.save();
    }
    
    match Args::parse().command {
        Command::Prov(args) => args.run(),
        Command::Country(args) => args.run(),
        
        Command::Cfg(args) => args.run(),
    };
}
