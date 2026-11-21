use anyhow::{Context, Result, bail};
use dirs::home_dir;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cli::PackageManager;

pub fn run(
    dirs: bool,
    installs: bool,
    package_manager: PackageManager,
    submodules: bool,
    dryrun: bool,
) -> Result<()> {
    println!("Starting `kaas init`");

    if dirs {
        create_directories()?;
    }

    if submodules {
        update_submodules()?;
    }

    if installs {
        println!("[INSTALLS] Installing packages");
        match package_manager {
            PackageManager::Apt => {
                println!("[INSTALLS] Using `apt` package manager");
                install_apt_packages()?
            }
        }

        // This is necessary on some older debian/ubuntu versions
        setup_bat_symlink()?;
        install_gf()?;

        change_shell()?;
        println!("[INSTALLS] Finished installing packages");
    }

    println!("`kaas init` finished succesfully.");
    Ok(())
}

fn would(dryrun: bool, msg: &str) {
    if dryrun {
        println!("[DRY-RUN] {msg}");
    }
}

fn home() -> Result<PathBuf> {
    home_dir().context("[HOME] Could not determine home directory somehow?")
}

fn create_directories() -> Result<()> {
    println!("[DIRECTORIES] Creating directories.");

    let home = home()?;

    // - Create ~/projects, ~/opt, ~/testing, ~/.local/bin and ~/.config
    let dirs = [
        home.join("projects"),
        home.join("opt"),
        home.join("testing"),
        home.join(".local/bin"),
        home.join(".config"),
    ];

    for dir in &dirs {
        if !dir.exists() {
            fs::create_dir_all(dir)
                .with_context(|| format!("Failed to create `{}`", dir.display()))?;
            println!("[DIRECTORIES] Created `{}`", dir.display());
        } else {
            println!("[DIRECTORIES] Directory `{}` already exists", dir.display());
        }
    }

    Ok(())
}

fn run_cmd(cmd: &str, arguments: &[&str]) -> Result<()> {
    println!("[RUNCMD] Running `{}` `{}`", cmd, arguments.join(" "));

    let status = Command::new(cmd)
        .args(arguments)
        .status()
        .with_context(|| {
            format!(
                "[RUNCMD] Failed to execute `{}` `{}`",
                cmd,
                arguments.join(" ")
            )
        })?;

    if !status.success() {
        bail!(
            "[RUNCMD] `{}` `{}` failed with status {}",
            cmd,
            arguments.join(" "),
            status
        );
    }

    Ok(())
}

fn update_submodules() -> Result<()> {
    println!("[SUBMODULES] Updating `git` submodules");

    // git submodule update --init --recursive
    run_cmd("git", &["submodule", "update", "--init", "--recursive"])?;

    println!("[SUBMODULES] Finished updating `git` submodules");

    Ok(())
}

fn install_apt_packages() -> Result<()> {
    println!("[APT INSTALLATION] Installing `apt` packages");

    // - Download curl, wget, build-essential, jq, fzf, libx11-dev, firefox, discord, tmux, clangd, cmake, mpv, gdb
    let packages = [
        "curl",
        "wget",
        "build-essential",
        "jq",
        "ripgrep",
        "fzf",
        "libx11-dev",
        "firefox",
        "tmux",
        "clangd",
        "cmake",
        "mpv",
        "gdb",
        "bat",
        "lua5.1",
        "neovim",
        "zsh",
    ];

    run_cmd("sudo", &["apt", "update"])?;
    let mut args = vec!["apt", "install"];
    args.extend(packages.iter().copied());
    run_cmd("sudo", &args)?;

    // Installing Discord & Spotify
    // These use .deb files so I need apt so I can't put them with the otherssssssssss

    install_discord()?;
    install_spotify()?;

    println!("[APT INSTALLATION] Finished installing `apt` packages");
    Ok(())
}

fn install_discord() -> Result<()> {
    println!("[DISCORD] Installing `Discord`");

    let deb = "/tmp/discord.deb";
    run_cmd(
        "wget",
        &[
            "-",
            deb,
            "https://discord.com/api/download?platform=linux&format=deb",
        ],
    )?;

    run_cmd("sudo", &["apt", "install", "-y", deb])?;
    let _ = fs::remove_file(deb);

    println!("[DISCORD] Finished installing `Discord`");
    Ok(())
}

fn install_spotify() -> Result<()> {
    println!(
        "[SPOTIFY] Look, bud, you're gonna have to install spotify yourself. They change the download link every day. I just can't do it anymore"
    );
    Ok(())
}

fn setup_bat_symlink() -> Result<()> {
    println!("[BAT] Setting up batcat symlink");

    let home = home()?;
    let link = home.join(".local/bin/bat");
    let target = Path::new("/usr/bin/batcat");

    if link.exists() {
        println!("[BAT] symlink already exists");
        return Ok(());
    }

    // Make sure ~/.local/bin exists
    if let Some(parent) = link.parent() {
        fs::create_dir_all(parent)?;
    }

    symlink(target, &link).with_context(|| {
        format!(
            "[BAT] Failed to create symlink `{}` -> `{}`",
            link.display(),
            target.display()
        )
    })?;

    println!("[BAT] Set up `batcat` symlink");
    Ok(())
}

fn install_gf() -> Result<()> {
    println!("[GF2] Installing `gf`");

    let home = home()?;
    let opt = home.join("opt");
    let gf_dir = opt.join("gf");

    if gf_dir.exists() {
        println!("[GF] `gf` already cloned at `{}`", gf_dir.display());
    } else {
        run_cmd(
            "git",
            &[
                "clone",
                "https://github.com/nakst/gf.git",
                gf_dir.to_str().unwrap(),
            ],
        )?;
    }

    println!("[GF] Building `gf`");
    let status = Command::new("./build.sh")
        .current_dir(&gf_dir)
        .status()
        .context("[GF] Failed to run `gf` `build.sh`")?;

    if !status.success() {
        bail!("[GF] `gf` build failed");
    }

    // Optionally put gf2 on PATH later (you can symlink it to ~/.local/bin)
    let gf2 = gf_dir.join("gf2");
    let link = home.join(".local/bin/gf");
    if gf2.exists() && !link.exists() {
        symlink(&gf2, &link)?;
        println!("[GF] Created symlink `{}` -> `{}`", link.display(), gf2.display());
    }

    println!("[GF2] Finished installing `gf`");
    Ok(())
}

fn change_shell() -> Result<()> {
    println!("[SHELL] Setting `zsh` to be default shell");

    let zsh_path = "/usr/bin/zsh"; // TODO: Change this to `which zsh`
    run_cmd("chsh", &["-s", zsh_path])?;

    println!("[SHELL] Changed shell");

    Ok(())
}
