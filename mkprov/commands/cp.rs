
use clap::Args;
use paradox_file::{Config, PdxFile};
use crate::commands;
use crate::common::Id;

#[derive(Debug, Args)]
pub struct CmdArgs {
    /// ID of province that will be used as source
    from_prov_id: u16,

    /// ID of province that will receive the copy
    to_prov_id: u16,
    
    #[arg(long, default_value_t = false)]
    /// executes cp-area alongside cp, getting both the area and the defines (default: false)
    with_area: bool
}

pub fn run(args: CmdArgs) {
    let cfg = Config::current();
    let from = PdxFile::inspect(
        &cfg, "history/provinces/", &Id(args.from_prov_id)).unwrap();
    let mut file = PdxFile::pull(
        &cfg, "history/provinces/", &Id(args.to_prov_id)).unwrap();

    file.contents = from;

    file.save();

    println!("Copied successfully!");
    
    if args.with_area {
        commands::mv_area::run(commands::mv_area::CmdArgs {
            from_prov_id: args.from_prov_id,
            to_prov_id: args.to_prov_id
        })
    }
}
