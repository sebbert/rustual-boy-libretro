#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

extern crate libc;
use libc::{ c_void, c_char, size_t };

extern crate rustual_boy_core;

use rustual_boy_core::vip;
use rustual_boy_core::vsu;
use rustual_boy_core::virtual_boy::{
	VirtualBoy
};

mod system_info;
use system_info::SystemInfo;

mod callbacks;
use callbacks::*;



pub struct Context {
}

impl Context {
	pub fn system_av_info(&self) -> SystemAvInfo {
		SystemAvInfo {
			geometry: SystemGameGeometry {
				base_width: vip::FRAMEBUFFER_RESOLUTION_X as u32,
				base_height: vip::FRAMEBUFFER_RESOLUTION_Y as u32,
				max_width: vip::FRAMEBUFFER_RESOLUTION_X as u32,
				max_height: vip::FRAMEBUFFER_RESOLUTION_Y as u32,

				// Optional
				aspect_ratio: 0.0
			},
			timing: SystemTiming {
				fps: 50.0, // TODO
				sample_rate: vsu::SAMPLE_RATE as f64
			}
		}
	}
}

#[repr(C)]
pub struct SystemGameGeometry {
	base_width: u32,
	base_height: u32,
	max_width: u32,
	max_height: u32,
	aspect_ratio: f32 
}

#[repr(C)]
pub struct SystemTiming {
	fps: f64,
	sample_rate: f64
}

#[repr(C)]
pub struct SystemAvInfo {
	geometry: SystemGameGeometry,
	timing: SystemTiming
}

static mut GLOBAL_CALLBACKS: Callbacks = Callbacks {
	environment_fn: None,
	video_refresh_fn: None,
	audio_sample_fn: None,
	audio_sample_batch_fn: None,
	input_poll_fn: None,
	input_state_fn: None
};

static mut GLOBAL_CONTEXT: *mut Context = 0 as *mut _;

unsafe fn get_context() -> &'static mut Context {
	&mut *GLOBAL_CONTEXT
}

unsafe fn set_context(context: Box<Context>) {
	GLOBAL_CONTEXT = Box::into_raw(context);
}

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
pub unsafe extern fn retro_get_system_av_info(av_info: *mut SystemAvInfo) {
	*av_info = get_context().system_av_info();
}

#[no_mangle]
pub unsafe extern fn retro_init() {

}

#[no_mangle]
pub unsafe extern fn retro_deinit() {
	
}

#[no_mangle]
pub extern fn retro_api_version() -> u32 { 1 }