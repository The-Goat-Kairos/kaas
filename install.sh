#!/usr/bin/env bash
set -euo pipefail

echo "=== Kaas Bootstrap ==="

# I would put these in scripts but I need them here
sudo apt update && sudo apt install -y git curl wget zsh build-essential

mkdir -p ~/projects ~/opt ~/.local/bin ~/.config

touch ~/todo.txt

if [[ ! -d "$HOME/kaas" ]]; then
    git clone https://github.com/The-Goat-Kairos/kaas.git ~/kaas
    cd ~/kaas
fi

git submodule update --init --recursive

for module in modules/*.sh; do
    echo "Running Module: $module"
    bash "$module"
done

./update.sh

echo "Kaas Installed! Run 'kaas update' to re-sync configs later."
