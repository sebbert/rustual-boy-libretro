extern crate libc;
use libc::c_char;

macro_rules! cstring {
    ($ex:expr) => {{
        (concat!($ex, "\0")).as_ptr() as *const c_char
    }}
}

#[repr(C)]
pub struct SystemInfo {
    pub library_name: *const i8,
    pub library_version: *const i8,
    pub valid_extensions: *const i8,
    pub need_fullpath: bool,
    pub block_extract: bool
}

impl SystemInfo {
	pub fn new() -> SystemInfo {
		SystemInfo {
			library_name: cstring!("Rustual Boy"),
			library_version: cstring!(env!("CARGO_PKG_VERSION")),
			valid_extensions: cstring!("vb"),
			need_fullpath: false,
			block_extract: false
		}
	}
}