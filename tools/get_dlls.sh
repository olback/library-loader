#!/bin/bash

LIBRARY_LOADER=library-loader.exe
LIBRARY_LOADER_PATH=$DIST
DLLS_PATH=$GTK_INSTALL_PATH

NEEDED=$(strings $LIBRARY_LOADER_PATH/$LIBRARY_LOADER | grep '\.dll$')

for dll in $NEEDED; do

    DLL_PATH="$DLLS_PATH/$dll"

    if [ -e $DLL_PATH ]; then

        cp $DLL_PATH $LIBRARY_LOADER_PATH

    fi


done
