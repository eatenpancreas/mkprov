use std::fs::File;
use std::io::Write;
use clap::Args;


#[derive(Debug, Args)]
pub struct SafeMakeProvsArgs {
    /// Default name for province
    #[arg(short = 'n', long, default_value = "Test")]
    name: String,

    #[arg(short = 'c', long, default_value = "sawabantu")]
    culture: String,

    #[arg(short = 'r', long, default_value = "shamanism")]
    religion: String,

    #[arg(short = 'a', long, default_value = "Cameroon")]
    capital: String,

    #[arg(short = 'd', long)]
    province_directory: String,

    #[arg(short, long)]
    starting_id: u16,

    #[arg(short, long)]
    ending_id: u16,
}

pub fn run(args: SafeMakeProvsArgs) {
    for id in args.starting_id..=args.ending_id {
        let name = &args.name;
        let culture = &args.culture;
        let religion = &args.religion;
        let capital = &args.capital;
        let dir = &args.province_directory;

        let text = format!(r#"
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
"#);
        match File::create(format!("{dir}/{id} - {name}.txt")) {
            Ok(mut f) => {
                if let Err(e) = f.write_all(text.as_bytes()) {
                    eprintln!("{}", e)
                }
            }
            Err(e) => eprintln!("{}", e)
        }
    }
}