use clap::{Args, Subcommand};

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
  pub fn run(self) {
    match self.cmd {
      Mk(args) => args.run(),
      Own(args) => args.run(),
      Cp(args) => args.run(),
      MvArea(args) => args.run(),
      Rn(args) => args.run(),
      Convert(args) => args.run(),
    }
  }
}