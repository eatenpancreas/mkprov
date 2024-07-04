
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
    pub fn run(self, cfg: &mut Config) {
        match self.method {
            Method::Set(args) => {
                if let Some(dir) = args.game_directory {
                    cfg.set_game_directory(Some(dir));
                }
                if let Some(dir) = args.mod_directory {
                    cfg.set_mod_directory(Some(dir));
                }
                cfg.save();

                println!("Saved changes!");
            }
            Method::OverrideAll(args) => {
                cfg.set_game_directory(args.game_directory);
                cfg.set_mod_directory(args.mod_directory);
                cfg.save();

                println!("Saved changes!");
            }
            Method::Echo => {
                cfg.echo_all_fields();
            }
        }
    }
}


