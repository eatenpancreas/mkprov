use itertools::Itertools;

use super::{Workspace, WorkspaceFile};
use std::{
    fs::{self, ReadDir},
    io::ErrorKind,
    path::Path,
};

impl Workspace {
    pub fn get_folder(&self, path: impl AsRef<Path>) -> Vec<WorkspaceFile<()>> {
        let read_source = fs::read_dir(self.game_location.as_ref().unwrap().join(&path));
        let read_mod = fs::read_dir(self.location.join(path));

        let mut v = match read_source {
            Ok(o) => self.get_folder_inner(o),
            Err(e) if e.kind() == ErrorKind::NotFound => vec![],
            Err(e) => panic!("{e}"),
        };

        let v_2 = match read_mod {
            Ok(o) => self
                .get_folder_inner(o)
                .into_iter()
                .filter(|f| !v.contains(f))
                .collect_vec(),
            Err(e) if e.kind() == ErrorKind::NotFound => vec![],
            Err(e) => panic!("{e}"),
        };

        v.extend(v_2);

        v
    }

    fn get_folder_inner(&self, read: ReadDir) -> Vec<WorkspaceFile<()>> {
        read.into_iter()
            .filter_map(|ent| {
                ent.ok().and_then(|ent| {
                    ent.metadata()
                        .is_ok_and(|m| m.is_file())
                        .then_some(self.get_any_file(ent.path().to_str()?))
                })
            })
            .collect()
    }
}
