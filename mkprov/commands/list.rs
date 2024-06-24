use clap::Args;

#[derive(Debug, Args)]
pub struct CmdArgs {
    /// amount of IDs are in every line
    #[arg(short, long, default_value_t = 5)]
    line_end_every: u16,

    /// Starting province ID
    #[arg(short, long)]
    starting_id: u16,

    /// amount of provinces to make
    #[arg(short, long)]
    count: u16,
}

pub fn run(args: CmdArgs) {
    println!();
    let mut i = 1;
    for id in args.starting_id..args.starting_id + args.count {
        print!("{id} ");
        if i % args.line_end_every == 0 {
            println!();
        }
        i += 1;
    }
    println!();
}
