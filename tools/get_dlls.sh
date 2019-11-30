#!/bin/bash

LIBRARY_LOADER=library-loader.exe
LIBRARY_LOADER_PATH=.
DLLS_PATH=dlls

NEEDED=$(strings $LIBRARY_LOADER | grep '\.dll$')

for dll in $NEEDED; do

    DLL_PATH="$DLLS_PATH/$dll"

    if [ -e $DLL_PATH ]; then

        cp $DLL_PATH $LIBRARY_LOADER_PATH

    fi


done
