extern crate rustual_boy_core;

use rustual_boy_core::{ vip, vsu };
use rustual_boy_core::virtual_boy::VirtualBoy;
use rustual_boy_core::rom::Rom;
use rustual_boy_core::sram::Sram;
use rustual_boy_core::time_source::TimeSource;
use rustual_boy_core::audio_frame_sink::AudioFrameSink;
use rustual_boy_core::video_frame_sink::VideoFrameSink;

use system_av_info::{
	SystemAvInfo,
	SystemGameGeometry,
	SystemTiming
};

pub struct Context {
	virtual_boy: VirtualBoy,
}

impl Context {
	pub fn new(rom: Rom, sram: Sram) -> Context {
		Context {
			virtual_boy: VirtualBoy::new(rom, sram),
		}
	}

	pub fn system_av_info(&self) -> SystemAvInfo {
		SystemAvInfo {
			geometry: SystemGameGeometry {
				base_width: vip::DISPLAY_RESOLUTION_X as u32,
				base_height: vip::DISPLAY_RESOLUTION_Y as u32,
				max_width: vip::DISPLAY_RESOLUTION_X as u32,
				max_height: vip::DISPLAY_RESOLUTION_Y as u32,

				// Optional
				aspect_ratio: 0.0
			},
			timing: SystemTiming {
				fps: 50.0,
				sample_rate: vsu::SAMPLE_RATE as f64
			}
		}
	}
}