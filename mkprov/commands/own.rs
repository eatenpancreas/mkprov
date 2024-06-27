
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
    let mut file = PdxFile::pull(&cfg, "history/provinces/", &Id(args.id));

    let tag = args.tag.to_uppercase();

    if !file.contents.mutate_kv("owner", 
        |kv| kv.set_value(tag.clone())) {
        file.contents.insert_kv(0,"owner", tag.clone())
    }

    if !file.contents.mutate_kv("controller", 
        |kv| kv.set_value(tag.clone())) {
        file.contents.insert_kv(1, "controller", tag.clone())
    }
    file.contents.insert_kv(1, "add_core", tag.clone());

    file.contents.retain(|field| !field.key_is("native_size")
        && !field.key_is("native_ferocity")
        && !field.key_is("native_hostileness")
    );
    
    file.save();

    println!("{tag} now owns {}!", args.id);
}
