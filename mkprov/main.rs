mod action;
mod query;

use action::ActionArgs;
use clap::{CommandFactory, Parser};
use mod_workspace::Workspace;
use query::QueryArgs;
use std::io;

fn main() {
    let workspace = match Workspace::load().unwrap() {
        Some(wk) => wk,
        None => Workspace::create().unwrap(),
    };

    if !atty::is(atty::Stream::Stdin) {
        let lines: Vec<String> = io::stdin()
            .lines()
            .collect::<Result<_, _>>()
            .expect("Failed to read stdin input");

        let args = ActionArgs::parse();
        if args.print_help {
            print_action_help();
        }

        ActionArgs::main(args.commands.unwrap(), lines, workspace);
    } else {
        let args = QueryArgs::parse();
        if args.action {
            print_action_help();
        }
        QueryArgs::main(args.kind.unwrap(), args.items, workspace);
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
