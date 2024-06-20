use clap::Args;

#[derive(Debug, Args)]
pub struct ListArgs {
    #[arg(short, long)]
    starting_id: u16,

    #[arg(short, long)]
    ending_id: u16,

    #[arg(short, long)]
    line_end_every: u16,
}

pub fn run(args: ListArgs) {
    println!();
    let mut i = 1;
    for id in args.starting_id..=args.ending_id {
        print!("{id} ");
        if i % args.line_end_every == 0 { println!(); }
        i += 1;
    }
    println!();
}