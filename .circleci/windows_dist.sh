#!/bin/bash

# https://gtk-rs.org/docs-src/tutorial/cross

GTK_INSTALL_PATH="/usr/x86_64-w64-mingw32/sys-root/mingw"
DIST="dist"

mkdir $DIST

# Exe
cp ./target/x86_64-pc-windows-gnu/release/library-loader.exe $DIST/library-loader.exe

# Dlls
cp $GTK_INSTALL_PATH/bin/*.dll $DIST

# Share
mkdir -p $DIST/share/glib-2.0/schemas
mkdir -p $DIST/share/icons

cp $GTK_INSTALL_PATH/share/glib-2.0/schemas/* $DIST/share/glib-2.0/schemas
cp -r $GTK_INSTALL_PATH/share/icons/* $DIST/share/icons

zip dist.zip dist

ls -lah dist
