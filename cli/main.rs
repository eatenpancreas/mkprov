pub mod commands {
    pub mod cfg;
    pub mod ls;
    pub mod prov;
}
mod cli_data;
mod common;

use ::common::Config;
use clap::{Parser, Subcommand};
use cli_data::CliData;
use inquire::Confirm;

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
    Prov(commands::prov::GroupArgs),

    // - base commands
    /// Changes project configs
    Cfg(commands::cfg::CmdArgs),
}

fn main() {
    let mut cli = CliData::new(Config::current().unwrap());

    if cli.config.is_first_time {
        let ans = Confirm::new("Are you sure you want to use mkprov for modding?")
            .with_default(false)
            .with_help_message(
                "mkprov is a in-beta tool designed for total overhaul mods for eu4.\
            There are a couple issues, particularly there is no support for comments, and as it \
            modifies files it will gradually delete all comments in files it touches. \
            Make sure you are okay with this before using the tool.",
            )
            .prompt()
            .unwrap_or(false);

        if !ans {
            return;
        }
        cli.config.is_first_time = false;
        cli.config.save();
    }

    match Args::parse().command {
        Command::Prov(args) => args.run(&cli),

        Command::Cfg(args) => args.run(&mut cli),
    };
}
