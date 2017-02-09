@ECHO OFF
SETLOCAL

CALL cargo build --release
CALL retroarch_debug --verbose -L %~dp0target\release\rustual_boy_libretro.dll %*