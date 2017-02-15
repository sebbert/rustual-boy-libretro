extern crate rustual_boy_core;
use rustual_boy_core::game_pad::{ Button, GamePad };

use callbacks::Callbacks;
use input::{ JoypadButton, AnalogStick };

pub fn map_input(callbacks: &'static Callbacks, game_pad: &mut GamePad) {
	game_pad.set_button_pressed(Button::A,              callbacks.joypad_button(JoypadButton::A));
	game_pad.set_button_pressed(Button::B,              callbacks.joypad_button(JoypadButton::B));
	game_pad.set_button_pressed(Button::L,              callbacks.joypad_button(JoypadButton::L));
	game_pad.set_button_pressed(Button::R,              callbacks.joypad_button(JoypadButton::R));
	game_pad.set_button_pressed(Button::Start,          callbacks.joypad_button(JoypadButton::Start));
	game_pad.set_button_pressed(Button::Select,         callbacks.joypad_button(JoypadButton::Select));

	let analog_threshold = (0x7fff as i16) / 2;

	let (left_x, left_y) = callbacks.analog_xy(AnalogStick::Left);
	game_pad.set_button_pressed(Button::LeftDPadLeft,   left_x < -analog_threshold);
	game_pad.set_button_pressed(Button::LeftDPadRight,  left_x > analog_threshold);
	game_pad.set_button_pressed(Button::LeftDPadUp,     left_y < -analog_threshold);
	game_pad.set_button_pressed(Button::LeftDPadDown,   left_y > analog_threshold);

	let (right_x, right_y) = callbacks.analog_xy(AnalogStick::Right);
	game_pad.set_button_pressed(Button::RightDPadLeft,  right_x < -analog_threshold);
	game_pad.set_button_pressed(Button::RightDPadRight, right_x > analog_threshold);
	game_pad.set_button_pressed(Button::RightDPadUp,    right_y < -analog_threshold);
	game_pad.set_button_pressed(Button::RightDPadDown,  right_y > analog_threshold);


}