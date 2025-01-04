use crate::cli_data::CliData;
use crate::commands;
use crate::common::ProvId;
use clap::Args;
use paradox_file::PdxFile;

#[derive(Debug, Args)]
pub struct CmdArgs {
    /// ID of province that will be used as source
    from_prov_id: u16,

    /// ID of province that will receive the copy
    to_prov_id: u16,

    #[arg(long, default_value_t = false)]
    /// executes cp-area alongside cp, getting both the area and the defines (default: false)
    with_area: bool,
}

impl CmdArgs {
    pub fn run(self, cli: &CliData) {
        let cfg = &cli.config;
        let from =
            PdxFile::inspect(&cfg, "history/provinces/", &ProvId(self.from_prov_id)).unwrap();
        let mut file = PdxFile::pull(&cfg, "history/provinces/", &ProvId(self.to_prov_id)).unwrap();

        file.contents = from;

        file.save();

        println!("Copied successfully!");

        if self.with_area {
            commands::prov::mv_area::CmdArgs {
                from_prov_id: self.from_prov_id,
                to_prov_id: self.to_prov_id,
            }
            .run(cli)
        }
    }
}
