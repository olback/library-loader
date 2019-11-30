#!/bin/bash

# https://gtk-rs.org/docs-src/tutorial/cross

GTK_INSTALL_PATH="/usr/x86_64-w64-mingw32/sys-root/mingw"
DIST="dist"

mkdir $DIST

# Exe
cp ./target/x86_64-pc-windows-gnu/release/library-loader.exe $DIST/library-loader.exe

# Dlls
cp $GTK_INSTALL_PATH/bin/*.dll $DIST

# Shemas
mkdir -p $DIST/share/glib-2.0/schemas
cp $GTK_INSTALL_PATH/share/glib-2.0/schemas/* $DIST/share/glib-2.0/schemas

# Icons
mkdir -p $DIST/share/icons
cp -r $GTK_INSTALL_PATH/share/icons/* $DIST/share/icons

# Windows 10 Theme
mkidr -p $DIST/share/themes
curl -sSL https://github.com/B00merang-Project/Windows-10/archive/master.zip -o /tmp/win10.zip
unzip /tmp/win10.zip
mv Windows-10-master $DIST/share/themes/Windows10

# Settings
mkdir -p $DIST/share/gtk-3.0
cat - > $DIST/share/gtk-3.0/settings.ini <<EOF
[Settings]
gtk-theme-name = Windows10
gtk-font-name = Segoe UI 9
gtk-xft-rgba = rgb
EOF


zip -r dist.zip dist
