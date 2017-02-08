extern crate rustual_boy_core;
use self::rustual_boy_core::{ vip, vsu };
use self::rustual_boy_core::virtual_boy::VirtualBoy;


use system_av_info::{
	SystemAvInfo,
	SystemGameGeometry,
	SystemTiming
};

pub struct Context {
	virtual_boy: VirtualBoy
}

impl Context {
	pub fn new(vb: VirtualBoy) -> Context {
		Context {
			virtual_boy: vb
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