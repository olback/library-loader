#!/bin/sh

set -e

LL_CORE=$(head ll-core/Cargo.toml -n 3 | tail -n 1)
LL_CLI=$(head ll-cli/Cargo.toml -n 3 | tail -n 1)
LL_GUI=$(head ll-gui/Cargo.toml -n 3 | tail -n 1)

if [ "$LL_CORE" != "$LL_CLI" ]; then
    echo "cli does not match core: Exptected $LL_CORE but got $LL_CLI"
    exit 1
fi

if [ "$LL_CORE" != "$LL_GUI" ]; then
    echo "gui does not match core: Exptected $LL_CORE but got $LL_GUI"
    exit 1
fi

echo "All versions match!"
