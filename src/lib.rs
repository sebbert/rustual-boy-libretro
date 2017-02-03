#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
extern crate libc;
use libc::{ c_void, c_char, size_t };

mod libretro;

mod callbacks;
use callbacks::*;

macro_rules! cstring {
    ($ex:expr) => {{
        (concat!($ex, "\0")).as_ptr() as *const c_char
    }}
}

impl SystemInfo {
	pub fn new() -> SystemInfo {
		SystemInfo {
			library_name: cstring!("SYSTEM_INFO_LIBRARY_NAME"),
			library_version: cstring!(env!("CARGO_PKG_VERSION")),
			valid_extensions: cstring!("vb")
		}
	}
}

#[derive(Default)]
pub struct Context {
	callbacks: Callbacks
}

#[repr(C)]
pub struct SystemInfo {
    pub library_name: *const i8,
    pub library_version: *const i8,
    pub valid_extensions: *const i8,
}

// libretro API
// ------------

#[no_mangle]
pub extern fn retro_set_environment(callback: EnvironmentCallback) {
	//TODO
}

#[no_mangle]
pub extern fn retro_set_video_refresh(callback: VideoRefreshCallback) {
	// TODO
}

#[no_mangle]
pub extern fn retro_set_audio_sample(callback: AudioSampleCallback) {
	// TODO
}

#[no_mangle]
pub extern fn retro_set_audio_sample_batch(callback: AudioSampleBatchCallback) {
	// TODO
}

#[no_mangle]
pub extern fn retro_set_input_poll(callback: InputPollCallback) {
	// TODO
}

#[no_mangle]
pub extern fn retro_set_input_state(callback: InputStateCallback) {
	// TODO
}

#[no_mangle]
pub unsafe extern fn retro_get_system_info(info: *mut SystemInfo) {
	*info = SystemInfo::new();
}


#[no_mangle]
pub unsafe extern fn retro_init() {

}

#[no_mangle]
pub extern fn retro_api_version() -> u32 { 1 }