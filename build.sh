#!/bin/bash
# This is the build iso script from PaleN1X but with some changes...

# Check if running as root
[ "$(id -u)" -ne 0 ] && {
    echo 'Please run as root'
    exit 1
}

# Check for correct number of arguments
if [ $# -ne 1 ]; then
    echo "Usage:  build.sh username"
    exit 1
}

# Build foxpass binary
cd ./foxpass/
sudo -u "$1" cargo build --release --target=x86_64-unknown-linux-gnu
cd ..
mkdir ./scripts
cp ./foxpass/target/x86_64-unknown-linux-gnu/release/foxpass ./scripts

# Alpine rootfs URL
ROOTFS="https://dl-cdn.alpinelinux.org/alpine/v3.20/releases/x86_64/alpine-minirootfs-3.20.3-x86_64.tar.gz"

echo $ROOTFS


# Clean previous work directory and create new structure
umount -v work/rootfs/{dev,sys,proc} >/dev/null 2>&1
rm -rf work
mkdir -pv work/{rootfs,iso/boot/grub}
cd work

# Extract rootfs and mount system directories
curl -sL "$ROOTFS" | tar -xzC rootfs
mount -vo bind /dev rootfs/dev
mount -vt sysfs sysfs rootfs/sys
mount -vt proc proc rootfs/proc
cp /etc/resolv.conf rootfs/etc

# Configure package repositories
cat << ! > rootfs/etc/apk/repositories
http://dl-cdn.alpinelinux.org/alpine/edge/main
http://dl-cdn.alpinelinux.org/alpine/edge/community
http://dl-cdn.alpinelinux.org/alpine/edge/testing
!

sleep 2

# Install required packages and configure services
cat << ! | chroot rootfs /usr/bin/env PATH=/usr/bin:/usr/local/bin:/bin:/usr/sbin:/sbin /bin/sh
apk update
apk upgrade
apk add bash alpine-base ncurses udev newt chntpw ntfs-3g fuse musl util-linux libgcc
apk add gcompat
apk add --no-scripts linux-lts linux-firmware-none
rc-update add bootmisc
rc-update add hwdrivers
rc-update add udev
rc-update add udev-trigger
rc-update add udev-settle
!

# Generate initramfs
chroot rootfs /usr/bin/env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/usr/local/bin \
	/sbin/mkinitfs -F "palen1x" -k -t /tmp -q $(ls rootfs/lib/modules)
mv -v rootfs/tmp/lib/modules rootfs/lib

# Update module dependencies
depmod -b rootfs $(ls rootfs/lib/modules)

# Configure system settings
echo 'foxpass' > rootfs/etc/hostname
echo "PATH=$PATH:$HOME/.local/bin" > rootfs/root/.bashrc # d
echo "export PALEN1X_VERSION='$VERSION'" > rootfs/root/.bashrc
echo 'clear' >> rootfs/root/.bashrc
echo '/usr/bin/foxpass' >> rootfs/root/.bashrc


# Unmount filesystems and finalize rootfs
umount -v rootfs/{dev,sys,proc}
cp -av ../inittab rootfs/etc
cp ../scripts/foxpass rootfs/usr/bin
chmod -v 755 rootfs/usr/local/bin/*
ln -sv sbin/init rootfs/init
ln -sv ../../etc/terminfo rootfs/usr/share/terminfo # fix ncurses

# Configure bootloader
cp -av rootfs/boot/vmlinuz-lts iso/boot/vmlinuz
cat << ! > iso/boot/grub/grub.cfg
insmod all_video
echo 'Foxpass V1 (Palen1x ISO maker)'
linux /boot/vmlinuz  quiet loglevel=3
initrd /boot/initramfs.xz
boot
!

# Create initramfs and ISO
pushd rootfs
rm -rf tmp/* boot/* var/cache/* etc/resolv.conf
find . | cpio -oH newc | xz -C crc32 --x86 -vz9eT$(nproc --all) > ../iso/boot/initramfs.xz
popd

# Create final ISO image
grub-mkrescue -o "foxpass.iso" iso --compress=xz