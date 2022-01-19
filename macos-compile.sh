#!/bin/zsh

set -e -o pipefail

BASEDIR=$(dirname "$0")

local RUSTC=$(which rustc)
local CARGO=$(which cargo)
local BREW=$(which brew)
local OPEN=$(which open)

local REQUIRED_PKG=("gtk+3" "atk" "gdk-pixbuf" "pango" "adwaita-icon-theme")
for PKG in $REQUIRED_PKG; do
    $BREW ls --versions $PKG || $BREW install $PKG
done

if [ $(uname -m) = "arm64" ]; then
  $CARGO build --release --target=aarch64-apple-darwin
  $OPEN $BASEDIR/target/aarch64-apple-darwin/release
else
  $CARGO build --release --target=x86_64-apple-darwin
  $OPEN $BASEDIR/target/x86_64-apple-darwin/release
fi

