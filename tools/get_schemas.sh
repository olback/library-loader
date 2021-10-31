#!/bin/bash

# Shemas
mkdir -p $DIST/share/glib-2.0/schemas
cp $GTK_INSTALL_PATH/share/glib-2.0/schemas/* $DIST/share/glib-2.0/schemas
