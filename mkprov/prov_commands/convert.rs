use clap::{Args, Subcommand};
use paradox_file::{Config, PdxFile};
use crate::common::ProvId;

#[derive(Debug, Args)]
pub struct CmdArgs {
  #[command(subcommand)]
  method: Method,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Method {
  Culture {
    /// ID of the province to be converted
    prov_id: u16,
    /// name of the culture
    culture: String,
  },
  Religion {
    /// ID of the province to be converted
    prov_id: u16,
    /// name of the religion
    religion: String,
  }
}

impl CmdArgs {
  pub fn run(self, cfg: &Config) {

    let (name, prov_id, key) = match self.method {
      Method::Culture { culture, prov_id } => (culture, prov_id, "culture"),
      Method::Religion { religion, prov_id } => (religion, prov_id, "religion"),
    };

    let mut file = PdxFile::pull(
      &cfg, "history/provinces/", &ProvId(prov_id)).unwrap();

    if !file.contents.mutate_kv(key, |kv| kv.set_value(&name)) {
      file.contents.insert_kv(0, key, &name);

      println!("{prov_id} is now {name} {key}!");
    }

    file.save();
  }
}
