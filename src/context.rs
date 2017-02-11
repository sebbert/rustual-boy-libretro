extern crate rustual_boy_core;

use rustual_boy_core::{ vip, vsu };
use rustual_boy_core::virtual_boy::VirtualBoy;
use rustual_boy_core::rom::Rom;
use rustual_boy_core::sram::Sram;
use rustual_boy_core::time_source::TimeSource;
use rustual_boy_core::sinks::{ Sink, AudioFrame, VideoFrame };

use retro_time_source::RetroTimeSource;
use callbacks::Callbacks;
use callback_sink::CallbackSink;
use system_av_info::{
	SystemAvInfo,
	SystemGameGeometry,
	SystemTiming
};

pub struct Context {
	virtual_boy: VirtualBoy,
	emulated_cycles: u64,
	time_source: RetroTimeSource
}

impl Context {
	pub fn new(rom: Rom, sram: Sram) -> Context {
		Context {
			virtual_boy: VirtualBoy::new(rom, sram),
			emulated_cycles: 0,
			time_source: RetroTimeSource::new()
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

	pub fn time_source_mut(&mut self) -> &mut RetroTimeSource {
		&mut self.time_source
	}

	pub fn run_frame(&mut self, callbacks: &'static Callbacks) {
		let mut audio_frame_sink = &mut CallbackSink(callbacks) as &mut Sink<AudioFrame>;
		let mut video_frame_sink = &mut CallbackSink(callbacks) as &mut Sink<VideoFrame>;

		// TODO: Record initial time and take difference
		let target_emulated_time_ns = self.time_source.time_ns();
		let target_emulated_cycles = target_emulated_time_ns / 50;

		while self.emulated_cycles < target_emulated_cycles {
			let (num_cycles, _) = self.virtual_boy.step(video_frame_sink, audio_frame_sink);

			self.emulated_cycles += num_cycles as _;
		}
	}
}