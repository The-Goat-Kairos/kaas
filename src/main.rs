mod cli;
mod init;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, EnvCommands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            dirs,
            installs,
            package_manager,
            submodules,
            dryrun,
        } => {
            println!(
                "kaas init: dirs={}, installs={}, package_manager={:?}, submodules={}, dryrun={}",
                dirs, installs, package_manager, submodules, dryrun
            );

            init::run(dirs, installs, package_manager, submodules, dryrun)?;
        }

        Commands::Env { action } => match action {
            EnvCommands::Apply => {
                println!("kaas env apply");
            }
            EnvCommands::Status => {
                println!("kaas env status");
            }
            EnvCommands::Commit => {
                println!("kaas env commit");
            }
        },
    }

    Ok(())
}
