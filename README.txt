# KAAS
### Short for Kairos' Awesome Automated System

git submodule add https://github.com/The-Goat-Kairos/nvim-config.git env/.config/nvim

Will be built in RUST and compiled to an executable that can be run on servers. If I want to modify the app, I would need rustc but otherwise I won't.


Capabilities:
    - Installation Script that will (kaas init):
        - Download curl, wget, build-essential, jq, fzf, libx11-dev, bat, firefox, discord, tmux, spotify, clangd, cmake, mpv, gdb
        - Flag to:
            - Create ~/projects, ~/opt, ~/testing, ~/.local/bin and ~/.config
            - git submodule update --init --recursive
        - ln -s /usr/bin/batcat ~/.local/bin/bat
        - Install ripgrep:
            curl -LO https://github.com/BurntSushi/ripgrep/releases/download/14.1.1/ripgrep_14.1.1-1_amd64.deb
            sudo dpkg -i ripgrep_14.1.1-1_amd64.deb
        - clone nakst/gf.git and build it in ~/opt
        - install lua5.1, neovim
        - Install zsh and change the shell

    - Update script that will (kaas update):
        - Sync all of my configuration files, practically mapping ~/kaas/env to ~ and ~/kaas/env/.config to ~/.config et cetera.
        - This should depend on whether I actually made those folders
        - kaas update # Does not check git before propagating changes
        - kaas update --check # checks git before propagating changes
        - Use a yaml-like file specifying source, target and mode="copy" or "symlink"
        - kaas env status # show missing or broken links or files
        - kaas env apply # commits dev changes

    - Command to manage my LaTeX files:
        - Symlink from ~/kaas/LaTeX to my current or specified directory
        - Update the symlinks?
        - Remove symlinks


    - Music player as in ~/Music that lets me play music, loop music, play a random song, play random songs on loop until I exit
    - Network
        - kaas net status
        - kaas net routes
        - kaas net check HOST

    - Diagnostics
        - kaas report # Collect a system snapshot
        - kaas report --json
        - kaas disk
        - kaas mounts

        - kaas top # Find high memory or CPU usage
        - kaas proc NAME
        - kaas resources # Inspect load, memory, swap and disk i/o use

    - Later I want to check the history of what kaas has done
        - kaas history

    - Later I want to be able to roll back versions
        - kaas env diff
        - kaas env apply
        - kaas env rollback

    - Later I might want to be able to control and operate virtual machines from here but that's too far
        - kaas host list
        - kaast host status NAME
