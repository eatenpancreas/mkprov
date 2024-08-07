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
    let mut cfg = Config::current().unwrap();
    if cfg.is_first_time {
        let ans = Confirm::new(
            "Are you sure you want to use mkprov for modding?"
        )
          .with_default(false)
          .with_help_message("mkprov is a in-beta tool designed for total overhaul mods for eu4.\
            There are a couple issues, particularly there is no support for comments, and as it \
            modifies files it will gradually delete all comments in files it touches. \
            Make sure you are okay with this before using the tool.")
          .prompt().unwrap_or(false);
        
        if !ans { return; }
        cfg.is_first_time = false;
        cfg.save();
    }
    
    match Args::parse().command {
        Command::Prov(args) => args.run(&cfg),
        Command::Country(args) => args.run(&cfg),
        
        Command::Cfg(args) => args.run(&mut cfg),
    };
}
