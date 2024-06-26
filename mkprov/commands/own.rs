use crate::common::{Config, Id, PdxFile};
use clap::Args;

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

    if !file.contents.mutate_key_val("owner", 
        |kv| kv.set_value(args.tag.clone())) {
        file.contents.push_field_kv("owner", args.tag.clone())
    }

    if !file.contents.mutate_key_val("controller", 
        |kv| kv.set_value(args.tag.clone())) {
        file.contents.push_field_kv("controller", args.tag)
    }
    
    file.save();
}
