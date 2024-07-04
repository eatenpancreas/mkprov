
use clap::Args;
use paradox_file::{Config, PdxFile};
use crate::common::Id;
use crate::prov_commands::mv_area;

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

impl CmdArgs {
    pub fn run(self) {
        let cfg = Config::current().unwrap();
        let from = PdxFile::inspect(
            &cfg, "history/provinces/", &Id(self.from_prov_id)).unwrap();
        let mut file = PdxFile::pull(
            &cfg, "history/provinces/", &Id(self.to_prov_id)).unwrap();

        file.contents = from;

        file.save();

        println!("Copied successfully!");

        if self.with_area {
            mv_area::CmdArgs {
                from_prov_id: self.from_prov_id,
                to_prov_id: self.to_prov_id
            }.run()
        }
    }
}

