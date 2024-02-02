#!/bin/sh

# update arch repository packages
pacman -Suy
# update all installed aur packages
paru -Sua
# install/update ohmyposh
curl -s https://ohmyposh.dev/install.sh | bash -s

