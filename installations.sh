#!/usr/bin/env bash

sudo apt install -y jq fzf libx11-dev bat firefox discord tmux bear clangd mpv gdb

ln -s /usr/bin/batcat ~/.local/bin/bat

# Ripgrep
curl -LO https://github.com/BurntSushi/ripgrep/releases/download/14.1.1/ripgrep_14.1.1-1_amd64.deb
sudo dpkg -i ripgrep_14.1.1-1_amd64.deb

pushd ~/opt
git clone https://github.com/nakst/gf.git
cd gf
./build.sh
popd

sudo apt install -y neovim lua5.1 luarocks
sudo luarocks install luacheck

sudo apt install -y zsh

hash -r

sudo chsh -s "$(which zsh)"
