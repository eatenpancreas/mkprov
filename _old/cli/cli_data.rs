use common::Config;
use std::io::{self, Read};

pub struct CliData {
    pub piped: Option<String>,
    pub config: Config,
}

impl CliData {
    pub fn new(config: Config) -> Self {
        let mut stdin = io::stdin().lock();

        let piped = if atty::is(atty::Stream::Stdin) {
            eprintln!("No data piped. Expecting input via a pipe.");
            None
        } else {
            let mut piped_data = String::new();
            stdin
                .read_to_string(&mut piped_data)
                .expect("Failed to read piped data");
            Some(piped_data)
        };

        Self { piped, config }
    }
}
