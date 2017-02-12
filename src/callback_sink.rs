extern crate libc;
use libc::c_void;

extern crate rustual_boy_core;
use rustual_boy_core::sinks::{
	Sink,
	AudioFrame,
	VideoFrame
};
use rustual_boy_core::vip::{
	DISPLAY_RESOLUTION_X,
	DISPLAY_RESOLUTION_Y
};

use ::callbacks::Callbacks;

pub struct CallbackSink(pub &'static Callbacks);

impl Sink<VideoFrame> for CallbackSink {
	fn append(&mut self, frame: VideoFrame) {
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

impl Sink<AudioFrame> for CallbackSink {
	fn append(&mut self, frame: AudioFrame) {
		let callbacks = self.0;

		let (left, right) = frame;
		callbacks.audio_sample(left, right);
	}
}