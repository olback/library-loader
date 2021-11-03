#!/bin/bash

set -e

if [ $UID != 0 ]; then
    echo "Must run as root (sudo)"
    exit 1
fi

# Verify binaries
sha256sum -c library-loader-cli.sha256
sha256sum -c library-loader-gui.sha256

# Copy binaries
cp library-loader-cli /usr/bin
cp library-loader-gui /usr/bin

# Copy desktop file & icon
desktop-file-install library-loader-gui.desktop
cp library-loader-icon.svg /usr/share/icons/hicolor/scalable/apps/net.olback.LibraryLoader.svg
gtk-update-icon-cache /usr/share/icons/hicolor

echo "Done"
