mod province_csv;

use mkprov_lib::workspace::{SaveFileError, Workspace, WorkspaceFile};
use pdxsyn::{
    Document,
    syntax::{ObjectLike, RootObject},
};
pub use province_csv::*;

use crate::common::CountryTag;

pub struct ProvinceHistories(Vec<WorkspaceFile<()>>);

impl ProvinceHistories {
    pub fn load(workspace: &Workspace) -> Self { Self(workspace.get_folder("history/provinces")) }
    pub fn find_id_inner(&self, id: u16) -> Option<(&WorkspaceFile<()>, &str)> {
        self.0.iter().find_map(|f| {
            f.path().file_name().and_then(|name| {
                let i = name
                    .to_str()
                    .and_then(|s| s.trim().starts_with(&id.to_string()).then_some((f, s)));

                i
            })
        })
    }

    pub fn find_id(&self, id: u16, workspace: &Workspace) -> Option<ProvinceHistory> {
        let (file, file_name) = self.find_id_inner(id)?;
        let province_name = file_name
            .split('-')
            .nth(1)
            .and_then(|n| n.trim().split('.').next())?
            .to_string();

        let file = file.clone().into_typed();
        let (document, obj) = file.load_either(&workspace).unwrap();

        Some(ProvinceHistory { _id: id, _name: province_name, file, document, obj })
    }
}

pub struct ProvinceHistory {
    _id: u16,
    _name: String,
    file: WorkspaceFile<(Document, RootObject)>,
    document: Document,
    obj: RootObject,
}

impl ProvinceHistory {
    pub fn set_owner(&mut self, owner: CountryTag) {
        match self.obj.get_first_mut(&mut self.document, "owner") {
            Some(set_owner) => set_owner
                .as_value()
                .unwrap()
                .replace(owner.to_string(), &mut self.document),
            None => {
                let idx = self.obj.first_index(&self.document, "culture").unwrap_or(0);
                self.obj
                    .insert(&mut self.document, idx, "owner", owner.to_string());
            }
        }
    }

    pub fn save(self, workspace: &Workspace) -> Result<(), SaveFileError<(Document, RootObject)>> {
        self.file.save((self.document, self.obj), workspace)
    }
}
