#!/bin/sh

# update arch repository packages
sudo pacman -Suy
# update all installed aur packages
paru -Sua
# install/update ohmyposh
curl -s https://ohmyposh.dev/install.sh | sudo bash -s

