extern crate rustual_boy_core;
use rustual_boy_core::game_pad::{ GamePad, Button };

use callbacks::Callbacks;

pub enum JoypadButton {
	B = 0,
	Y = 1,
	Select = 2,
	Start = 3,
	Up = 4,
	Down = 5,
	Left = 6,
	Right = 7,
	A = 8,
	X = 9,
	L = 10,
	R = 11,
	L2 = 12,
	R2 = 13,
	L3 = 14,
	R3 = 15
}

#[allow(dead_code)]
pub enum RetroDeviceType {
	None = 0,
	Joypad = 1,
	Mouse = 2,
	Keyboard = 3,
	Lightgun = 4,
	Analog = 5,
	Pointer = 6
}

impl Callbacks {
	pub fn joypad_button(&self, button: JoypadButton) -> bool {
		0 != self.input_state(0, RetroDeviceType::Joypad as u32, 0, button as u32)
	}
}