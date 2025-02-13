use std::fs;

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
    match workspace.game.unwrap() {
        Game::Eu4 => mkprov_eu4::main(workspace),
    }
}

fn temp_workdir_create() -> Workspace {
    // let n = fs::read_dir("target/test_outputs/")
    //     .map(|read| read.count())
    //     .unwrap_or(0);

    let mut mod_dir = std::env::current_dir().unwrap();
    mod_dir.push(format!("target/test_outputs/"));
    fs::create_dir_all(&mod_dir).unwrap();
    let mut wk = Workspace::custom_create(mod_dir).unwrap();

    let mut source_dir = std::env::current_dir().unwrap();
    source_dir.push("test_files/eu4");
    wk.game_location = Some(source_dir);
    wk.game = Some(Game::Eu4);

    wk
}
