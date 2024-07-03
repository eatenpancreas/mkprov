//TODO


use clap::Args;
use paradox_file::{Config, PdxFile, LocalisationFile, DefinitionCsv};
use crate::common::Id;

#[derive(Debug, Args)]
pub struct CmdArgs {
  /// ID of province to be renamed
  prov_id: u16,
  
  /// What to rename province into
  rename_into: String,
  
  // rename capital to
  #[arg(short, long)]
  capital: Option<String>,

  // priority(?) of province localisation
  #[arg(short, long)]
  priority: Option<u8>
}

pub fn run(args: CmdArgs) {
  let cfg = Config::current();
  let mut yml = LocalisationFile::load_localisation(&cfg).unwrap();
  yml.replace_or_add_key_name(args.prov_id, args.rename_into.clone(), args.priority);
  yml.save();

  let mut def = DefinitionCsv::load(&cfg).unwrap();
  def.rename(args.prov_id, args.rename_into.clone());
  def.save();
  
  let mut file = PdxFile::pull(
    &cfg, "history/provinces/", &Id(args.prov_id)).unwrap();
  
  if let Some(capital) = args.capital {
    if !file.contents.mutate_kv("capital",
      |kv| kv.set_value(capital.clone())) {
      file.contents.insert_kv(0,"capital", capital.clone())
    }
  }
  
  file.rename_prov_name(args.prov_id, args.rename_into).unwrap();

  file.save();
}