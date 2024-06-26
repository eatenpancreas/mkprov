
use clap::Args;
use std::fs::File;
use std::io::Write;
use paradox_file::Config;

#[derive(Debug, Args)]
pub struct CmdArgs {
    /// Default name for province
    #[arg(short = 'n', long, default_value = "Test")]
    name: String,

    /// Default culture for province
    #[arg(short = 'u', long, default_value = "sawabantu")]
    culture: String,

    /// Default religion for province
    #[arg(short = 'r', long, default_value = "shamanism")]
    religion: String,

    /// Default capital for province
    #[arg(short = 'a', long, default_value = "Cameroon")]
    capital: String,

    /// Starting province ID
    #[arg(short, long)]
    starting_id: u16,

    /// amount of provinces to make
    #[arg(short, long)]
    count: u16,
}

pub fn run(args: CmdArgs) {
    for id in args.starting_id..args.starting_id + args.count {
        let cfg = Config::current();

        let dir = cfg.require_mod_directory();
        let name = &args.name;
        let culture = &args.culture;
        let religion = &args.religion;
        let capital = &args.capital;

        let text = format!(
            r#"
#{id} - {name}
culture = {culture}
religion = {religion}
capital = {capital}
trade_goods = unknown
hre = no
base_tax = 1
base_production = 1
base_manpower = 1
native_size = 90
native_ferocity = 4
native_hostileness = 12



discovered_by = KON
discovered_by = NDO
discovered_by = LOA
discovered_by = sub_saharan
"#
        );
        match File::create(format!("{dir}/history/provinces/{id} - {name}.txt")) {
            Ok(mut f) => {
                if let Err(e) = f.write_all(text.as_bytes()) {
                    eprintln!("{}", e)
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
