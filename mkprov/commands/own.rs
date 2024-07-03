
use clap::Args;
use paradox_file::{Config, PdxFile};
use crate::common::Id;

#[derive(Debug, Args)]
pub struct CmdArgs {
    /// Tag that will own province
    owner_tag: String,

    /// province ID
    prov_id: u16,
}

pub fn run(args: CmdArgs) {
    let cfg = Config::current();
    let mut file = PdxFile::pull(
        &cfg, "history/provinces/", &Id(args.prov_id)).unwrap();

    let tag = args.owner_tag.to_uppercase();

    if !file.contents.mutate_kv("owner", 
        |kv| kv.set_value(&tag)) {
        file.contents.insert_kv(0,"owner", &tag)
    }

    if !file.contents.mutate_kv("controller", 
        |kv| kv.set_value(&tag)) {
        file.contents.insert_kv(1, "controller", &tag)
    }
    file.contents.insert_kv(1, "add_core", &tag);

    file.contents.retain(|field| !field.key_is("native_size")
        && !field.key_is("native_ferocity")
        && !field.key_is("native_hostileness")
    );
    
    file.save();

    println!("{tag} now owns {}!", args.prov_id);
}
