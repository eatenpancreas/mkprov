use clap::{Args, Subcommand};

use crate::cli_data::CliData;

#[derive(Debug, Args)]
pub struct CmdArgs {
    #[command(subcommand)]
    method: Method,
}

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[arg(short, long)]
    mod_directory: Option<String>,
    #[arg(short, long)]
    game_directory: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Method {
    /// sets configs if mentioned
    Set(ConfigArgs),
    /// overrides configs, if not mentioned it will delete
    OverrideAll(ConfigArgs),
    /// echo's current config
    Echo,
}

impl CmdArgs {
    pub fn run(self, cli: &mut CliData) {
        match self.method {
            Method::Set(args) => {
                if let Some(dir) = args.game_directory {
                    cli.config.set_game_directory(Some(dir));
                }
                if let Some(dir) = args.mod_directory {
                    cli.config.set_mod_directory(Some(dir));
                }
                cli.config.save();

                println!("Saved changes!");
            }
            Method::OverrideAll(args) => {
                cli.config.set_game_directory(args.game_directory);
                cli.config.set_mod_directory(args.mod_directory);
                cli.config.save();

                println!("Saved changes!");
            }
            Method::Echo => {
                cli.config.echo_all_fields();
            }
        }
    }
}
