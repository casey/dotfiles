#!/bin/sh

# resize partition table
(
# delete partition 3
echo d
echo 3
# recreate partition 3
echo n
echo
echo
echo
# write
echo w
) | fdisk /dev/vda

resize2fs /dev/vda3

echo "Done resising the filesystem, please reboot."
