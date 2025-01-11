pub mod provs;

use self::Command::*;
use crate::cli_data::CliData;
use clap::Args;
use clap::Subcommand;

#[derive(Debug, Args)]
pub struct GroupArgs {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// lists provinces
    Provs(provs::CmdArgs),
}

impl GroupArgs {
    pub fn run(self, cli: &CliData) {
        match self.cmd {
            Provs(args) => args.run(cli),
        }
    }
}
