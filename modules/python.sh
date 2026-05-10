#!/usr/bin/env bash

echo "Installing Python"

sudo apt-get install software-poperties-common
sudo apt-get-repository ppa:deadsnakes/ppa
sudo apt-get update
sudo apt install -y python3.14

echo "Python module done."
