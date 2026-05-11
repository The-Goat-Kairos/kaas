#!/usr/bin/env bash

echo "Installing the Core"

sudo apt install -y ripgreq jq fzf bat firefox discord tmux awk bear clangd mpv

ln -s /usr/bin/batcat ~/.local/bin/bat

pushd ~/opt
git clone https://github.com/nakst/gf.git
cd gf
./build.sh
popd

echo "CORE module done."
