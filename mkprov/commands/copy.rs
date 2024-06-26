
use clap::Args;
use paradox_file::{Config, PdxFile};
use crate::common::Id;

#[derive(Debug, Args)]
pub struct CmdArgs {
    /// ID that will be copied
    #[arg(short, long)]
    from_id: u16,

    /// province ID that will be copied to
    #[arg(short, long)]
    to_id: u16,
}

pub fn run(args: CmdArgs) {
    let cfg = Config::current();
    let from = PdxFile::inspect(Id(args.from_id), &cfg, "history/provinces/").unwrap();
    let mut file = PdxFile::pull(Id(args.to_id), &cfg, "history/provinces/").unwrap();

    file.contents = from;

    file.save();

    println!("Copied successfully!");
}
