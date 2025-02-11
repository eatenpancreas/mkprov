pub mod action;
pub mod query;

#[cfg(test)]
mod test;

use action::ActionArgs;
use clap::{CommandFactory, Parser};
use itertools::Itertools;
use mkprov_lib::workspace::Workspace;
use query::{QueryArgs, QueryOutput};
use std::io::{self, Read};

fn main() {
    let workspace = temp_workdir_create();
    // let workspace = match Workspace::load().unwrap() {
    //     Some(wk) => wk,
    //     None => Workspace::create().unwrap(),
    // };

    if !atty::is(atty::Stream::Stdin) {
        let mut file = String::new();
        io::stdin()
            .read_to_string(&mut file)
            .expect("Failed to read stdin input");

        let query_output = file.split_whitespace().map(|s| QueryOutput::parse(s));

        let args = ActionArgs::parse();
        if args.print_help {
            print_action_help();
        }

        ActionArgs::main(args.commands.unwrap(), query_output, workspace);
    } else {
        let args = QueryArgs::parse();
        if args.action {
            print_action_help();
        }

        let out = QueryArgs::main(args.kind.unwrap(), args.items, workspace);
        println!("{}", out.iter().map(ToString::to_string).collect_vec().join(" "));
    }
}

fn print_action_help() {
    let mut action = ActionArgs::command().help_template(
        "\
            {before-help}{name} {version}\n\
            {about-with-newline}\n\
            {usage-heading} {usage}\n\
            \n\
            {all-args}{after-help}
        ",
    );
    action.print_help().unwrap();
    std::process::exit(0);
}

fn temp_workdir_create() -> Workspace {
    let p = tempdir::TempDir::new("").unwrap().into_path();
    let mut wk = Workspace::custom_create(p).unwrap();

    let mut source_dir = std::env::current_dir().unwrap();
    source_dir.push("test_files/eu4");
    wk.game_location = source_dir;

    wk
}
