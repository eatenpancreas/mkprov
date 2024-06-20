use clap::Args;
use rand::Rng;
use crate::lib::Color;

#[derive(Debug, Args)]
pub struct DefinitionListArgs {
    /// Default name for province
    #[arg(short = 'n', long, default_value = "Test")]
    name: String,

    #[arg(short, long)]
    starting_id: u16,

    #[arg(short, long)]
    ending_id: u16,
}

pub fn run(args: DefinitionListArgs) {
    let name = args.name;
    let mut col = Color::random();

    println!();
    for id in args.starting_id..=args.ending_id {
        println!("{id};{};{};{};{name};x", col.r(), col.g(), col.b());
        col.shift();
    }
}