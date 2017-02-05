#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
extern crate libc;
use libc::{ c_void, c_char, size_t };

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
}

#[repr(C)]
pub struct SystemInfo {
    pub library_name: *const i8,
    pub library_version: *const i8,
    pub valid_extensions: *const i8,
}

static mut GLOBAL_CALLBACKS: Callbacks = Callbacks {
	environment_fn: None,
	video_refresh_fn: None,
	audio_sample_fn: None,
	audio_sample_batch_fn: None,
	input_poll_fn: None,
	input_state_fn: None
};

// libretro API
// ------------

#[no_mangle]
pub unsafe extern fn retro_set_environment(callback: EnvironmentCallback) {
	GLOBAL_CALLBACKS.environment_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_set_video_refresh(callback: VideoRefreshCallback) {
	GLOBAL_CALLBACKS.video_refresh_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_set_audio_sample(callback: AudioSampleCallback) {
	GLOBAL_CALLBACKS.audio_sample_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_set_audio_sample_batch(callback: AudioSampleBatchCallback) {
	GLOBAL_CALLBACKS.audio_sample_batch_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_set_input_poll(callback: InputPollCallback) {
	GLOBAL_CALLBACKS.input_poll_fn = Some(callback);
}

#[no_mangle]
pub unsafe extern fn retro_set_input_state(callback: InputStateCallback) {
	GLOBAL_CALLBACKS.input_state_fn = Some(callback);
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