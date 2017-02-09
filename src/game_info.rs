extern crate libc;
use self::libc::{ c_char, c_void, size_t};

extern crate std;
use std::slice;

#[repr(C)]
pub struct GameInfo {
	path: *const c_char,
	data: *const c_void,
	size: size_t,
	meta: *const c_char
}