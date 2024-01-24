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
#swift
paru swift-bin
paru swift-language
#arm-linux-gnueabihf
paru arm-linux-gnueabihf-binutils
paru arm-linux-gnueabihf-gcc
paru arm-linux-gnueabihf-gdb
paru arm-linux-gnueabihf-glibc
paru arm-linux-gnueabihf-glibc-headers
paru arm-linux-gnueabihf-ncurses
paru arm-linux-gnueabihf-musl
#riscv64-unknown-elf
paru riscv64-unknown-elf-binutils
paru riscv64-unknown-elf-gcc
paru riscv64-unknown-elf-newlib
paru riscv64-unknown-elf-picolibc
#arm-linux-gnueabi
paru arm-linux-gnueabi-binutils
paru arm-linux-gnueabi-gcc
#msp430-elf
paru msp430-elf-binutils
paru msp430-elf-gcc
paru msp430-elf-newlib
paru msp430-elf-libstdc++
#i386-elf
paru i386-elf-binutils
paru i386-elf-gcc
paru i386-elf-gdb
#mips64-elf
paru mips64-elf-binutils
paru mips64-elf-gcc
paru mips64-elf-newlib
#m68k-elf
paru m68k-elf-binutils
paru m68k-elf-gcc
paru m68k-elf-gdb
paru m68k-elf-newlib
paru m68k-elf-toolchain

