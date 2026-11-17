#!/usr/bin/env bash
set -euo pipefail

echo "=== Kaas Bootstrap ==="

# I would put these in scripts but I need them here
echo "Installing essential programs"
sudo apt update && sudo apt install -y git curl wget zsh build-essential
echo "Done installing essentials"

echo "Creating necessary folders"
mkdir -p ~/Projects ~/opt ~/.local/bin ~/.config
echo "Done reating necessary folders"


touch ~/todo.txt


# We're assuming we're in the ~/Kaas folder
git submodule update --init --recursive


echo "Installing programs"
./installations.sh
echo "Done installing programs"


./update.sh

echo "Kaas Installed! Run 'kaas update' to re-sync configs later."
