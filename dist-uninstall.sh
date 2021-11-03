#!/bin/bash

set -e

if [ $UID != 0 ]; then
    echo "Must run as root (sudo)"
    exit 1
fi

# Remove binaries
rm /usr/bin/library-loader-cli
rm /usr/bin/library-loader-gui

# Copy desktop file & icon
rm /usr/share/applications/library-loader-gui.desktop
rm /usr/share/icons/hicolor/scalable/apps/net.olback.LibraryLoader.svg

echo "Done"
