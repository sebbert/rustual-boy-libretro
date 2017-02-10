#!/bin/sh

# This script builds rustual-boy-libretro in release mode and runs it in RetroArch
# All args are passed to RetroArch
#
# Note: RetroArch must be in $PATH

cd "$(dirname "$0")"
HERE=`pwd`

case "$OSTYPE" in
	darwin*) CORE_FILENAME='librustual_boy_libretro.dylib' ;;
	 linux*) CORE_FILENAME='librustual_boy_libretro.so' ;;
	  msys*) echo "Error: Retroarch CLI doesn't work in msys, run test.bat from cmd.exe"; exit 1 ;;
	      *) echo "Error: Unsupported OS"; exit 1 ;; 
esac

cargo build --release
retroarch --verbose -L "$HERE/target/release/$CORE_FILENAME" $@