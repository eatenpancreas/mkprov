use clap::{Args, Subcommand};
use self::Command::*;

#[derive(Debug, Args)]
pub struct GroupArgs {
  #[command(subcommand)]
  pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
  
}

impl GroupArgs {
  pub fn run(self) {
    match self.cmd {
      
    }
  }
}