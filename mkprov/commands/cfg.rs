
use clap::{Args, Subcommand};
use paradox_file::Config;

#[derive(Debug, Args)]
pub struct CmdArgs {
    #[command(subcommand)]
    method: Method,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Method {
    /// sets configs if mentioned
    Set(Config),
    /// overrides configs, if not mentioned it will delete
    OverrideAll(Config),
    /// echo's current config
    Echo,
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
