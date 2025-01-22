use clap::{Parser, Subcommand, ValueEnum};

/// The action part of Mkprov
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(name = "mkprov (action)")]
#[command(bin_name = "mkprov")]
#[command(override_usage = "...input... | mkprov <COMMAND>")]
#[command(disable_help_flag(true))]
#[command(version, about)]
pub struct ActionArgs {
    #[clap(subcommand)]
    pub commands: Option<ActionCommands>,
    /// Print help (after the | pipe. Alternatively, before the pipe, use --action to get this message)
    #[clap(long = "help", short = 'h')]
    pub print_help: bool,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ActionCommands {
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
pub enum MoveTo {
    Area,
    Tradenode,
    Continent,
}

impl ActionArgs {
    pub fn main(commands: ActionCommands, items: Vec<String>) { println!("{items:?}") }
}
