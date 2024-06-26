
use clap::Args;
use paradox_file::{Config, PdxFile};
use crate::common::Id;

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
    let mut file = PdxFile::pull(Id(args.id), cfg, "history/provinces/").unwrap();

    if !file.contents.mutate_kv("owner", 
        |kv| kv.set_value(args.tag.clone())) {
        file.contents.insert_kv(0,"owner", args.tag.clone())
    }

    if !file.contents.mutate_kv("controller", 
        |kv| kv.set_value(args.tag.clone())) {
        file.contents.insert_kv(1, "controller", args.tag)
    }
    
    file.save();
}
