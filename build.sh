#!/bin/bash

set -e

TARGET=${TARGET-x86_64-unknown-linux-gnu}
if [ "$TARGET" != "" ]; then
    TGT="--target $TARGET"
fi
VERSION=debug
if [ "$1" == "--release" ]; then
    VERSION=release
fi


# Runtimetest must be compiled in its dorectory and is
# not a part of youki workspace. For the reasoning behind this,
# please check the docs and readme

cargo build --verbose $TGT $1
cd ./runtimetest
cargo build --verbose $TGT $1
cd ..

cp target/$TARGET/$VERSION/youki .
cp target/$TARGET/$VERSION/ociplex .
cp target/$TARGET/$VERSION/integration_test ./youki_integration_test
cp runtimetest/target/$TARGET/$VERSION/runtimetest ./runtimetest_tool

BASEPATH=$(pwd)

for t in crates/ociplex/examples/*.toml; do
    OUT=$(basename $t .toml).ociplex
    sed "s!@BASEPATH@!$BASEPATH!" < $t > $OUT
    chmod +x $OUT
done
