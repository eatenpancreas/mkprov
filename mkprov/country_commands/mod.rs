use clap::{Args, Subcommand};

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