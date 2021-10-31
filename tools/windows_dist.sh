#!/bin/bash

# https://gtk-rs.org/docs-src/tutorial/cross

GTK_INSTALL_PATH="/usr/x86_64-w64-mingw32/sys-root/mingw"
DIST="dist"

mkdir $DIST
cp ./target/x86_64-pc-windows-gnu/release/library-loader.exe $DIST/library-loader.exe
./tools/get_dlls.sh
./tools/get_schemas.sh
./tools/get_icons.sh
./tools/get_theme.sh
./tools/settings.sh
zip -r $DIST.zip $DIST
