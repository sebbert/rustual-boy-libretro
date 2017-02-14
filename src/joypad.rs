extern crate rustual_boy_core;
use rustual_boy_core::game_pad::{ GamePad, Button };

use callbacks::Callbacks;

pub enum JoypadButton
{
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

pub enum RetroDeviceType
{
	None = 0,
	Joypad = 1
}

pub fn update(callbacks: &'static Callbacks, game_pad: &mut GamePad, joypad_button: JoypadButton, rb_button: Button) {
	let state = callbacks.input_state(0, RetroDeviceType::Joypad as u32, 0, joypad_button as u32);
	game_pad.set_button_pressed(rb_button, state > 0);
}

pub fn update_game_pad(callbacks: &'static Callbacks, game_pad: &mut GamePad) {
	update(callbacks, game_pad, JoypadButton::A,      Button::A);
	update(callbacks, game_pad, JoypadButton::B,      Button::B);
	update(callbacks, game_pad, JoypadButton::L,      Button::L);
	update(callbacks, game_pad, JoypadButton::R,      Button::R);
	update(callbacks, game_pad, JoypadButton::Start,  Button::Start);
	update(callbacks, game_pad, JoypadButton::Select, Button::Select);

	update(callbacks, game_pad, JoypadButton::Left,   Button::LeftDPadLeft);
	update(callbacks, game_pad, JoypadButton::Right,  Button::LeftDPadRight);
	update(callbacks, game_pad, JoypadButton::Up,     Button::LeftDPadUp);
	update(callbacks, game_pad, JoypadButton::Down,   Button::LeftDPadDown);

	update(callbacks, game_pad, JoypadButton::R2,     Button::RightDPadLeft);
	update(callbacks, game_pad, JoypadButton::R3,     Button::RightDPadRight);
	update(callbacks, game_pad, JoypadButton::L2,     Button::RightDPadUp);
	update(callbacks, game_pad, JoypadButton::L3,     Button::RightDPadDown);
}