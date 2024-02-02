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
#loongarch64-linux-gnu
paru loongarch64-linux-gnu-binutils
paru loongarch64-linux-gnu-linux-api-headers
paru loongarch64-linux-gnu-glibc
paru loongarch64-linux-gnu-gcc
#xtensa-esp32-elf
paru xtensa-esp32-elf-binutils
paru xtensa-esp32-elf-newlib
paru xtensa-esp32-elf-gcc
#mips-elf
paru mips-elf-binutils
paru mips-elf-gcc
#sh-elf
paru sh-elf-binutils
paru sh-elf-gcc
paru sh-elf-newlib
paru sh-elf-binutils-casio
paru sh-elf-gcc-casio
#rwth-psp
paru rwth-psp-binutils
paru rwth-psp-gcc
paru rwth-psp-libc
#mips64-linux-gnu
paru mips64-linux-gnu-binutils
paru mips64-linux-gnu-linux-api-headers
paru mips64-linux-gnu-gcc
paru mips64-linux-gnu-glibc
#mips64-elf
paru mips64-elf-binutils
paru mips64-elf-gcc
paru mips64-elf-newlib
#i686-elf
paru i686-elf-binutils
paru i686-elf-gcc
paru i686-elf-gdb
#i386-elf
paru i386-elf-binutils
paru i386-elf-gcc
paru i386-elf-gdb
#mips64-ultra-elf
paru mips64-ultra-elf-binutils
paru mips64-ultra-elf-gcc-stage1
paru mips64-ultra-elf-newlib
paru mips64-ultra-elf-gcc
#arm-frc-linux-gnueabi
paru arm-frc-linux-gnueabi-binutils
paru arm-frc-linux-gnueabi-eglibc
paru arm-frc-linux-gnueabi-gcc
paru arm-frc-linux-gnueabi-wpilib
#aarch64-ppp-elf
paru aarch64-ppp-elf
#osxcross
paru osxcross-git
#ps3toolchain
paru ps3-env
paru ps3-ppu-binutils
paru ps3-ppu-gcc
paru ps3-spu-binutils
paru ps3-spu-gcc
paru ps3-psl1ght
paru ps3toolchain
paru ps3-zlib
paru ps3-zstd
paru ps3-ffmpeg-libs
paru ps3libraries

# Install OhMyPosh

curl -s https://ohmyposh.dev/install.sh | bash -s

