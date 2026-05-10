#!/usr/bin/env bash

echo "Installing Neovim"

sudo apt install -y neovim lua51 luarocks
sudo luarocks install luacheck

echo "Neovim module done."
