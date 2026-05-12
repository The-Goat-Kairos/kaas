#!/usr/bin/env bash

echo "Installing ZSH"

sudo apt install -y zsh

hash -r

sudo chsh -s "$(which zsh)"

echo "ZSH module done."
