#[cfg(test)]
mod test;

use std::{fs, io};

use clap::Parser;
use crossterm::{
    execute,
    style::{Print, PrintStyledContent, Stylize},
};
use mkprov_lib::workspace::{Game, Workspace};

fn main() -> () {
    if let Err(err) = inner_main() {
        let styled = "Mkprov error: ".to_string().bold().dark_red();
        let mut stdout = io::stdout();
        execute!(stdout, PrintStyledContent(styled), Print(err), Print('\n')).unwrap();
    }
}

fn inner_main() -> anyhow::Result<()> {
    let workspace = temp_workdir_create();
    // let workspace = match Workspace::load().unwrap() {
    //     Some(wk) => wk,
    //     None => Workspace::create().unwrap(),
    // };
    //
    match workspace.game.unwrap() {
        Game::Eu4 => match mkprov_eu4::Args::parse().command {
            mkprov_eu4::Command::Query { query, actions } => {
                let ids = query.query_province_ids(&workspace)?;

                // let chosen = Selection::new(
                //     "(ESC to quit) No exact match was found, but similar province names exist: ",
                //     similar_scores.iter().map(|(name, _)| capitalize_first_letter(name)),
                // )
                // .style(SelectionStyle::default())
                // .display()
                // .unwrap();
                actions.execute(ids, &workspace)?;
            }
            mkprov_eu4::Command::Make(_) => {}
        },
    }

    Ok(())
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
