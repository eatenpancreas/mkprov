use clap::{Parser, Subcommand, ValueEnum};

/// The action part of Mkprov
#[derive(Parser, Debug)]
pub struct ActionArgs {
    #[clap(subcommand)]
    commands: ActionCommands,
}

#[derive(Subcommand, Clone, Debug)]
enum ActionCommands {
    /// Moves the item
    Move {
        /// where to move to
        to: MoveTo,
        value: String,
    },
    /// Deletes the items
    Delete,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum MoveTo {
    Area,
    Tradenode,
    Continent,
}
