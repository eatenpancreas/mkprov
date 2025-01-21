mod action;
mod query;

use action::ActionArgs;
use clap::Parser;
use query::QueryArgs;
use std::io::{self, Read};

fn main() {
    if !atty::is(atty::Stream::Stdin) {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read stdin");
        println!("Got piped input: {}", buffer);

        let args = ActionArgs::parse();
        println!("{:?}", args);
    } else {
        let args = QueryArgs::parse();
        println!("{:?}", args);
    }
}
