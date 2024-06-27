use clap::Args;
use paradox_file::{Config, Field, PdxFile};

#[derive(Debug, Args)]
pub struct CmdArgs {
    /// ID of province that has the right area
    #[arg(short, long)]
    from_area_prov_id: u16,

    /// province ID that is going to change area
    #[arg(short, long)]
    to_prov_id: u16,
}

pub fn run(args: CmdArgs) {
    let cfg = Config::current();
    let mut file = PdxFile::pull(&cfg, "map/", &"area.txt");

    let mut areas = file.contents.get_child_objects_mut();

    for area in &mut areas {
        area.retain(|f| !f.key_is(args.to_prov_id))
    }

    for area in areas {
        if area.find_mut(|x| x.key_is(args.from_area_prov_id)).is_some() {
            area.push(Field::new_literal(args.to_prov_id));
            break;
        }
    }

    file.save();

    println!("Area moved successfully!");
}