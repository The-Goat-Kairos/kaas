# KAAS
### Short for Kairos' Awesome Automated System

git submodule add https://github.com/The-Goat-Kairos/nvim-config.git env/.config/nvim


Current plan
Create an installation script and folder that I can just git clone and then run and everything works. In bash.
It must download all of the things I need (i.e. all apt programs; I have some bash files that add repositories or whatever is needed to install certain things but I don't really like that system).
I am considering trying to use 'apt' or another package manager less but for now we can ignore that.
I want a folder, called 'env', which holds all of the configuration files I want on my system. These are things like:
*current folder*/env/.zshrc would go to ~/.zshrc and ./env/.zsh_profile would go to \~/.zsh_profile.
It will also contain a folder .config which holds itself all folders and files which should go into \~/.config (i.e. my tmux, mpv, clangd, etc). There can also be other folders like ./env/.local which should go to \~/.local.

./env/.config/nvim is a git submodule (https://github.com/The-Goat-Kairos/nvim-config) so it is a seperate repo that I push and commit to seperately so we have to take that into account

I want scripts to configure everything for the initial setup, and then also a script (perhaps the same script with an argument or perhaps 2 seperate scripts) which then takes everything from this folder and resends it through my system so I can just call it and it updates my system. I want it to be a smart system I can easily expand and perhaps expand to having multiple (virtual) systems that I might be able to ssh in. Later I might want to be able to control those servers from my main host or whatever. it should be flexible and also easy to install.

Also it should create folders ~/projects and ~/opt.

Steps for new system would be
Sudo apt install git
*configure git*
git clone .....
./configure or whatever

