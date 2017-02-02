#[macro_use]
extern crate bitflags;
extern crate libc;
use libc::size_t;
use std::ptr;
use std::ffi::CString;

mod libretro;

const RETRO_SYSTEM_INFO_LIBRARY_NAME: &'static str = "Rustual Boy";
// TODO: Consider using actual Rustual Boy version.
const RETRO_SYSTEM_INFO_LIBRARY_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const RETRO_SYSTEM_INFO_VALID_EXTENSIONS: &'static str = "vb";
