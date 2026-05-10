#!/usr/bin/env bash
set -euo pipefail

echo "=== Syncing Kaas Configs ==="

ENV_DIR="$HOME/kaas/env"

# Symlink root dotfiles
for file in "$ENV_DIR"/.[!.]*; do
    if [[ -f "$file" ]]; then
        target="$HOME/$(basename "file")"
        ln -sf "$file" "$target"
        echo "Linked $target"
    fi
end

# Symlink .config contents
if [[ -d "$ENV_DIR/.config" ]]; then
    mkdir -p "$HOME/.config"
    for item in "$ENV_DIR/.config/"*; do
        if [[ -e "$item" ]]; then
            target="$HOME/.config/$(basename "$item")"
            ln -sf "$item" "$target"
            echo "Linked $target"
        fi
    done
fi

# Symlink .local contents
if [[ -d "$ENV_DIR/.local" ]]; then
    rsync -av --ignore-existing "$ENV_DIR/.local/" "$HOME/.local/"
fi

echo "Configuration Synced."
