use clap::Args;
use paradox_file::{Config, Field, PdxFile};

#[derive(Debug, Args)]
pub struct CmdArgs {
    /// ID of province that has the right area
    pub(crate) from_prov_id: u16,

    /// province ID that is going to change area
    pub(crate) to_prov_id: u16,
}

impl CmdArgs {
    pub fn run(self, cfg: &Config) {
        let mut file = PdxFile::pull(&cfg, "map/", &"area.txt").unwrap();

        let mut areas = file.contents.get_child_objects_mut();

        for area in &mut areas {
            area.retain(|f| !f.key_is(self.to_prov_id))
        }

        for area in areas {
            if area.find_mut(|x| x.key_is(self.from_prov_id)).is_some() {
                area.push(Field::new_literal(self.to_prov_id));
                break;
            }
        }

        file.save();

        println!("Area moved successfully!");
    }
}