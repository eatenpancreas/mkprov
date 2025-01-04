use crate::cli_data::CliData;
use clap::Args;

#[derive(Debug, Args)]
pub struct CmdArgs {}

impl CmdArgs {
    pub fn run(self, cli: &CliData) {}
}
