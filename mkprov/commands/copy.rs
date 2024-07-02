
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
    let from = PdxFile::inspect(&cfg, "history/provinces/", &Id(args.from_id)).unwrap();
    let mut file = PdxFile::pull(&cfg, "history/provinces/", &Id(args.to_id)).unwrap();

    file.contents = from;

    file.save();

    println!("Copied successfully!");
}
