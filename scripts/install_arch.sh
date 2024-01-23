#!/bin/sh

ROOT=$1
BOOT=$2
HOME=$3
MEDIA=$4
SWAP=$5

if [[ -z "$ROOT" ]]; then
	echo "A root partition is required"
	exit
elif [[ -z "$BOOT" ]]; then
	echo "A boot partition is required"
	exit
fi

mkfs.fat -F 32 $BOOT
mkfs.ext4 $ROOT

mount $ROOT /mnt
mount --mkdir $BOOT /mnt/boot


if [ -z "$HOME" ]; then
	mkfs.ext4 $HOME
	mount --mkdir $HOME /mnt/home
fi
if [ -z "$MEDIA" ]; then
	mkfs.ext4 $MEDIA
	mount --mkdir $MEDIA /mnt/media
fi
if [ -z "$SWAP" ]; then
	mkswap $SWAP
	swapon $SWAP
fi

pacstrap -K base linux-hardened linux-firmware base-devel lvm2 git
genfstab -U /mnt >> /mnt/etc/fstab

echo "ohseyme" >> /mnt/etc/hostname

