//TODO


use clap::Args;
use paradox_file::{Config, PdxFile, YmlFile};
use crate::common::Id;

#[derive(Debug, Args)]
pub struct CmdArgs {
  /// province ID
  #[arg(short, long)]
  id: u16,
  
  /// Rename province to
  #[arg(short, long)]
  to: String,
  
  // rename capital to
  #[arg(short, long)]
  capital: Option<String>,

  // priority(?) of province localisation
  #[arg(short, long)]
  priority: Option<u8>
}

pub fn run(args: CmdArgs) {
  let cfg = Config::current();
  let mut yml = YmlFile::load_localisation(&cfg).unwrap();
  yml.replace_or_add_key_name(args.id, args.to.clone(), args.priority);
  
  let mut file = PdxFile::pull(&cfg, "history/provinces/", &Id(args.id)).unwrap();
  
  if let Some(capital) = args.capital {
    if !file.contents.mutate_kv("capital",
      |kv| kv.set_value(capital.clone())) {
      file.contents.insert_kv(0,"capital", capital.clone())
    }
  }
  
  file.rename_prov_name(args.id, args.to).unwrap();
  file.save();
}