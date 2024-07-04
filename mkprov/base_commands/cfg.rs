
use clap::{Args, Subcommand};
use paradox_file::Config;

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

#[derive(Debug, Subcommand, Clone)]
pub enum Method {
    /// sets configs if mentioned
    Set(ConfigArgs),
    /// overrides configs, if not mentioned it will delete
    OverrideAll(ConfigArgs),
    /// echo's current config
    Echo,
}

impl CmdArgs {
    pub fn run(self) {
        match self.method {
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
}


