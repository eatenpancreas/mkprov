use mkprov_lib::workspace::{Game, Workspace};

#[cfg(test)]
mod test;

fn main() {
    let workspace = temp_workdir_create();
    // let workspace = match Workspace::load().unwrap() {
    //     Some(wk) => wk,
    //     None => Workspace::create().unwrap(),
    // };
    //
    match workspace.game {
        Game::Eu4 => mkprov_eu4::main(workspace),
    }
}

fn temp_workdir_create() -> Workspace {
    let p = tempdir::TempDir::new("").unwrap().into_path();
    let mut wk = Workspace::custom_create(p).unwrap();

    let mut source_dir = std::env::current_dir().unwrap();
    source_dir.push("test_files/eu4");
    wk.game_location = source_dir;

    wk
}
