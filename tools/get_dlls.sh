#!/bin/bash

LIBRARY_LOADER_PATH=$(readlink -f $DIST)
DLLS_PATH="$GTK_INSTALL_PATH/bin"

cp $DLLS_PATH/*.dll $LIBRARY_LOADER_PATH

# CWD=$(pwd)
# ALWAYS_NEEDED="libgcc_s_seh-1.dll libpixman-1-0.dll libpng16-16.dll zlib1.dll libepoxy-0.dll libintl-8.dll libgmodule-2.0-0.dll"

# cd $DLLS_PATH
# cp $ALWAYS_NEEDED $LIBRARY_LOADER_PATH
# cd $CWD

# LIBRARY_LOADER=library-loader.exe
# NEEDED=$(strings $LIBRARY_LOADER_PATH/$LIBRARY_LOADER | grep '\.dll$')

# for dll in $NEEDED; do

#     DLL_PATH="$DLLS_PATH/$dll"

#     if [ -e $DLL_PATH ]; then

#         cp $DLL_PATH $LIBRARY_LOADER_PATH

#     fi


# done
