#!/bin/zsh

set -e -o pipefail

echo "On Construction."
false;

BASEDIR=$(dirname "$0")

MACHINE_ARCH=$(uname -m)

local RUSTC=$(which rustc)
local CARGO=$(which cargo)
local OPEN=$(which open)
local ARCH=$(which arch)

if [ $MACHINE_ARCH = "arm64" ]; then
  vared -p "aarch64 detected. Specify x86_64 homebrew path: " -c BREW
else
  local BREW=$(which brew)
fi

local REQUIRED_PKG=("gtk+3" "atk" "gdk-pixbuf" "pango" "adwaita-icon-theme")
for PKG in $REQUIRED_PKG; do
  if [ $MACHINE_ARCH = "arm64" ]; then
    $ARCH --x86_64 $BREW ls --versions $PKG || $ARCH --x86_64 $BREW install $PKG
  else
    $BREW ls --versions $PKG || $BREW install $PKG
  fi
done

$CARGO build --release --target=x86_64-apple-darwin

$OPEN $BASEDIR/target/aarch64-apple-darwin/release
