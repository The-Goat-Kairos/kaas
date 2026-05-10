#!/usr/bin/env bash

echo "Installing ZSH"

sudo apt install -y zsh

if [[ "$SHELL" != *zsh* ]]; then
    sudo chsh -s "$(which zsh)" "$USER"
fi

echo "ZSH module done."
