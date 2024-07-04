use clap::{Args, Subcommand};
use paradox_file::Config;

pub mod own;
pub mod cp;
pub mod mv_area;
pub mod mk;
pub mod rn;
pub mod convert;

use self::Command::*;

#[derive(Debug, Args)]
pub struct GroupArgs {
  #[command(subcommand)]
  pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
  /// Makes a list of non-owned, basic provinces in MOD/history/provinces. makes definitions, etc
  Mk(mk::CmdArgs),
  /// Converts a province to an owner tag
  Own(own::CmdArgs),
  /// Copies a province history to another.
  /// Optionally also uses cp-area to copy both
  Cp(cp::CmdArgs),
  /// Moves a province from an area to another
  MvArea(mv_area::CmdArgs),
  /// Renames a province
  Rn(rn::CmdArgs),
  /// Converts a province to a religion or culture
  Convert(convert::CmdArgs),
}

impl GroupArgs {
  pub fn run(self, cfg: &Config) {
    match self.cmd {
      Mk(args) => args.run(cfg),
      Own(args) => args.run(cfg),
      Cp(args) => args.run(cfg),
      MvArea(args) => args.run(cfg),
      Rn(args) => args.run(cfg),
      Convert(args) => args.run(cfg),
    }
  }
}