#!/bin/sh

# Install packages
pacman -S - < pkglist.txt

# Setup AUR helper
mkdir /home/Downloads
cd /home/Downloads
git clone https://aur.archlinux.org/paru.git
cd paru && makepkg -si && cd ..
paru --gendb

# Install AUR packages

paru hfsprogs
paru reiser4progs
paru zfs-utils
paru mergerfs
paru mhddfs
paru unionfs-fuse

