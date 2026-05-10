#!/usr/bin/env bash

echo "Installing the Core"

sudo apt install -y ripgreq jq fzf bat firefox discord tmux

ln -s /usr/bin/batcat ~/.local/bin/bat

echo "CORE module done."
