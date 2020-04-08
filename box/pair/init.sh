#!/bin/sh
#
# Adapted from https://eipi.xyz/blog/nixos-on-vultr/
#
# Install NixOS on a Vultr VPS

set -euxo pipefail

CONFIG_URL='https://raw.githubusercontent.com/casey/local/master/box/pair/configuration.nix'

umount /dev/vda*

# create partitions (with 2G swap)
(
echo g

# swap
echo n
echo
echo
echo +2GB
echo t
echo
echo 19

# bios boot (for grub)
echo n
echo
echo
echo +16MB
echo t
echo
echo 4

# /
echo n
echo
echo
echo

echo w
) | fdisk /dev/vda

fdisk -l /dev/vda

# enable swap
mkswap -f /dev/vda1
swapon /dev/vda1
free -h

# wait
sleep 5

# create filesystem and mount
mkfs.ext4 /dev/vda3 -Lroot
mount /dev/vda3 /mnt

nix-channel --add https://nixos.org/channels/nixos-unstable nixos

# generate Nixos config
nixos-generate-config --root /mnt

echo "System configuration.nix:"
curl "$CONFIG_URL" | tee /mnt/etc/nixos/configuration.nix

# install Nixos
nixos-install

# unmount
sync
umount /dev/vda3

echo "Done. Now reboot via \"Remove ISO\" on the Vultr web UI."
