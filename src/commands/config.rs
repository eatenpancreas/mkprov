use clap::{Args, Subcommand};
use crate::common::Config;

#[derive(Debug, Args)]
pub struct CmdArgs {
    #[command(subcommand)]
    method: Method,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Method {
    Set(Config),
    OverrideAll(Config),
    Echo
}

pub fn run(args: CmdArgs) {
    match args.method {
        Method::Set(cfg) => {
            let mut current_config = Config::current();
            current_config.set_fields(cfg);
            current_config.save();

            println!("Saved changes!");
        }
        Method::OverrideAll(cfg) => {
            let mut current_config = Config::current();
            current_config.override_all_fields(cfg);
            current_config.save();

            println!("Saved changes!");
        }
        Method::Echo => {
            let current_config = Config::current();
            current_config.echo_all_fields();
        }
    }
}