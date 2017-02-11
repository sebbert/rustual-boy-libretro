extern crate libc;
use libc::{c_void, size_t};

extern crate rustual_boy_core;
use rustual_boy_core::video_frame_sink::VideoFrameSink;
use rustual_boy_core::audio_frame_sink::AudioFrameSink;
use rustual_boy_core::vip::{
	DISPLAY_RESOLUTION_X,
	DISPLAY_RESOLUTION_Y
};

use ::callbacks::Callbacks;

pub struct CallbackSink(pub &'static Callbacks);

impl VideoFrameSink for CallbackSink {
	fn append(&mut self, frame: (Box<[u8]>, Box<[u8]>)) {
		let callbacks = self.0;

		let frame_ptr = Box::into_raw(frame.0) as *mut c_void;

		callbacks.video_refresh(
			frame_ptr,
			DISPLAY_RESOLUTION_X as u32,
			DISPLAY_RESOLUTION_Y as u32,
			DISPLAY_RESOLUTION_X);

		unsafe {
			Box::from_raw(frame_ptr);
		}
	}
}

impl AudioFrameSink for CallbackSink {
	fn append(&mut self, frame: (i16, i16)) {
		let callbacks = self.0;

		let (left, right) = frame;
		callbacks.audio_sample(left, right);
	}
}