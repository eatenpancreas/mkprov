use crate::common::Color;
use clap::Args;

#[derive(Debug, Args)]
pub struct CmdArgs {
    /// Default name for province
    #[arg(short = 'n', long, default_value = "Test")]
    name: String,

    /// Starting province ID
    #[arg(short, long)]
    starting_id: u16,

    /// amount of provinces to make
    #[arg(short, long)]
    count: u16,
}

pub fn run(args: CmdArgs) {
    let name = args.name;
    let mut col = Color::random();

    println!();
    for id in args.starting_id..args.starting_id + args.count {
        println!("{id};{};{};{};{name};x", col.r(), col.g(), col.b());
        col.shift();
    }
}
