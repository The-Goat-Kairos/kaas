use anyhow::Result;
use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "kaas",
    about = "Kairos' Awesome Automated System!",
    after_help = "Display help for individual commands with `kaas help <COMMAND>`"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize your system.
    #[command(long_about = "Long initialize")]
    Init {
        /// Create directories ~/projects, ~/opt, ~/testing, ~/.local/bin and ~/.config.
        #[arg(short, long, action = ArgAction::Set, default_value_t = true)]
        dirs: bool,

        /// Install the default applications.
        ///
        /// Includes curl, wget, build-essential, jq, fzf, libx11-dev, bat,
        /// firefox, discord, tmux, spotify, clangd, cmake, mpv, gdb,
        /// ripgrep, and neovim.
        #[arg(short, long, action = ArgAction::Set, default_value_t = true)]
        installs: bool,

        /// Set the package manager.
        #[arg(long, default_value = "apt")]
        package_manager: String,
    },

    /// Manage your environment according to the configuration.
    Env {
        #[command(subcommand)]
        action: EnvCommands,
    },
}

#[derive(Subcommand)]
enum EnvCommands {
    /// Apply environment changes according to the configuration.
    Apply,
    /// Show the status of environment changes.
    Status,
    /// Automatically commit environment changes.
    Commit,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            dirs,
            installs,
            package_manager,
        } => println!(
                "kaas init (dirs={dirs}, installs={installs}, package_manager={package_manager})"
        ),

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
