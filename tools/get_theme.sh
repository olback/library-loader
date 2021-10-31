#!/bin/bash

# Windows 10 Theme
mkdir -p $DIST/share/themes
curl -sSL https://github.com/B00merang-Project/Windows-10/archive/master.zip -o /tmp/win10.zip
unzip /tmp/win10.zip
mv Windows-10-master $DIST/share/themes/Windows10
