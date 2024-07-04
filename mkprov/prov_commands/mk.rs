
use clap::Args;
use std::fs::File;
use std::io::Write;
use rand::Rng;
use paradox_file::{Color, Config, DefinitionCsv, Field, Object, PdxFile, RGBShift};

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

    /// amount of provinces to make
    count: u16,
}

impl CmdArgs {
    pub fn run(self) {
        let cfg = Config::current();
        let dir = cfg.require_mod_directory();

        let province = province_def(&self);
        let name = &self.name;

        let mut area_file = PdxFile::pull(
            &cfg, "map/", &"area.txt").unwrap();
        let mut default_file = PdxFile::pull(
            &cfg, "map/", &"default.map").unwrap();

        let mut def = DefinitionCsv::load(&cfg).unwrap();

        let mut rng = rand::thread_rng();
        let area_name = format!("generated_area_{}", rng.gen_range(0..u16::MAX));
        let mut area_ids = vec![];
        let mut id = def.max_id();
        let mut col = Color::random();
        let rgb_shift = RGBShift::random();

        default_file.contents.mutate_kv("max_provs", |kv| {
            kv.set_value(id + self.count)
        });

        for _ in 0..self.count {
            id += 1;

            def.push(id, col, name.clone());

            area_ids.push(Field::new_literal(id));

            match File::create(format!("{dir}/history/provinces/{id} - {name}.txt")) {
                Ok(mut f) => {
                    if let Err(e) = f.write_all(province.as_bytes()) {
                        eprintln!("{}", e)
                    }
                }
                Err(e) => eprintln!("{}", e),
            }

            col.shift(rgb_shift);
        }

        area_file.contents.push(Field::new(area_name, Object::new(area_ids, 1)));
        area_file.save();
        default_file.save();
        def.save();
    }
}


pub fn province_def(args: &CmdArgs) -> String {
    let culture = &args.culture;
    let religion = &args.religion;
    let capital = &args.capital;

    format!(r#"
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
"#)
}