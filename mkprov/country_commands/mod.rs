use clap::{Args, Subcommand};
use paradox_file::Config;

#[derive(Debug, Args)]
pub struct GroupArgs {
  #[command(subcommand)]
  pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
  
}

impl GroupArgs {
  pub fn run(self, _cfg: &Config) {
    match self.cmd {
      
    }
  }
}