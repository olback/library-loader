#!/bin/bash

# Settings
mkdir -p $DIST/share/gtk-3.0
cat - > $DIST/share/gtk-3.0/settings.ini <<EOF
[Settings]
gtk-theme-name = Windows10
gtk-font-name = Segoe UI 9
gtk-xft-rgba = rgb
EOF
