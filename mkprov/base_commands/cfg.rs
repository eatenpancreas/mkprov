
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
    pub fn run(self) {
        match self.method {
            Method::Set(cfg) => {
                let mut current_config = Config::current().unwrap();
                if let Some(dir) = cfg.game_directory {
                    current_config.set_game_directory(Some(dir));
                }
                if let Some(dir) = cfg.mod_directory {
                    current_config.set_mod_directory(Some(dir));
                }
                current_config.save();

                println!("Saved changes!");
            }
            Method::OverrideAll(cfg) => {
                let mut current_config = Config::current().unwrap();
                current_config.set_game_directory(cfg.game_directory);
                current_config.set_mod_directory(cfg.mod_directory);
                current_config.save();

                println!("Saved changes!");
            }
            Method::Echo => {
                let current_config = Config::current().unwrap();
                current_config.echo_all_fields();
            }
        }
    }
}


