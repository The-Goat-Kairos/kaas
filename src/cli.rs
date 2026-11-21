use clap::{ArgAction, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "kaas",
    about = "Kairos' Awesome Automated System!",
    after_help = "Display help for individual commands with `kaas help <COMMAND>`"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initializes your system
    Init {
        /// Create directories ~/projects, ~/opt, ~/testing, ~/.local/bin and ~/.config
        #[arg(short, long, action = ArgAction::Set, default_value_t = true)]
        dirs: bool,

        /// Install the default applications:
        ///
        /// (curl, wget, build-essential, jq, fzf, libx11-dev, bat,
        /// firefox, discord, tmux, spotify, clangd, cmake, mpv, gdb,
        /// ripgrep, and neovim)
        #[arg(short, long, action = ArgAction::Set, default_value_t = true)]
        installs: bool,

        /// Set the package manager
        #[arg(long, default_value = "apt")]
        package_manager: PackageManager,

        /// Update and initialize git submodules:
        ///
        /// git submodule update --init --recursive
        #[arg(long, default_value_t = true)]
        submodules: bool,

        /// Do a dryrun (e.g. do not actually install anything)
        #[arg(long, default_value_t = false)]
        dryrun: bool,
    },

    /// Manages your environment according to the configuration
    Env {
        #[command(subcommand)]
        action: EnvCommands,
    },


    // There are more commands here later
}

#[derive(Debug, Clone, ValueEnum)]
pub enum PackageManager {
    Apt,
}

#[derive(Subcommand)]
pub enum EnvCommands {
    /// Apply environment changes according to the configuration
    Apply,
    /// Show the status of environment changes
    Status,
    /// Automatically commit all environment changes
    Commit,
}
