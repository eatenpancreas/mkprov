use crate::common::{Config, Province};
use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct CmdArgs {
    /// Tag that will own province
    #[arg(short, long)]
    tag: String,

    /// province ID
    #[arg(short, long)]
    id: u16,
}

pub fn run(args: CmdArgs) {
    let cfg = Config::current();
    let mod_dir = PathBuf::from(cfg.require_mod_directory());
    let game_dir = PathBuf::from(cfg.require_game_directory());

    let mut prov = Province::pull(args.id, mod_dir, game_dir).unwrap();
    // prov.set_owner(args.tag);
    prov.save();
}
