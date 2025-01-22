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
    /// Moves the item.
    /// Aliases = m
    #[clap(aliases = ["m", "move"])]
    MoveTo {
        /// where to move to
        to: MoveTo,
        /// name of the thing to move to
        name: String,
    },
    /// Sets the climate
    SetClimate {
        climate: ClimateType,
    },
    SetMonsoon {
        monsoon: Severity,
    },
    SetWinter {
        climate: Severity,
    },
    /// Deletes the items.
    /// Aliases = d
    #[clap(alias = "d")]
    Delete,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum MoveTo {
    Area,
    /// Moves to a tradenode.
    /// Aliases = tn
    #[clap(alias = "tn")]
    Tradenode,
    Continent,
    Climate,
    Monsoon,
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum ClimateType {
    Tropical,
    Arid,
    Arctic,
    #[default]
    Normal,
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum Severity {
    Normal,
    Mild,
    Severe,
    #[default]
    None,
}

impl ActionArgs {
    pub fn main(commands: ActionCommands, items: Vec<String>) { println!("{items:?}") }
}
