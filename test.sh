#!/bin/sh

# This script builds rustual-boy-libretro in release mode and runs it in RetroArch
# All args are passed to RetroArch
#
# Note: RetroArch must be in $PATH

cd "$(dirname "$0")"
HERE=`pwd`

cargo build --release
retroarch -vL "$HERE/target/release/librustual_boy_libretro.dylib" $@