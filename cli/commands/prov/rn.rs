use crate::{cli_data::CliData, common::ProvId};
use clap::Args;
use paradox_file::{DefinitionCsv, LocalisationFile, PdxFile};

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
    priority: Option<u8>,
}

impl CmdArgs {
    pub fn run(self, cli: &CliData) {
        let cfg = &cli.config;
        let mut yml = LocalisationFile::load_localisation(&cfg).unwrap();
        yml.replace_or_add_key_name(self.prov_id, self.rename_into.clone(), self.priority);
        yml.save();

        let mut def = DefinitionCsv::load(&cfg).unwrap();
        def.rename(self.prov_id, self.rename_into.clone());
        def.save();

        let mut file = PdxFile::pull(&cfg, "history/provinces/", &ProvId(self.prov_id)).unwrap();

        if let Some(capital) = self.capital {
            if !file
                .contents
                .mutate_kv("capital", |kv| kv.set_value(capital.clone()))
            {
                file.contents.insert_kv(0, "capital", capital.clone())
            }
        }

        file.rename_prov_name(self.prov_id, self.rename_into)
            .unwrap();

        file.save();
    }
}
