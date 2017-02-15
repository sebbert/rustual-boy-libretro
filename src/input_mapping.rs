extern crate rustual_boy_core;
use rustual_boy_core::game_pad::{ Button, GamePad };

use callbacks::Callbacks;
use input::JoypadButton;

pub fn map_input(callbacks: &'static Callbacks, game_pad: &mut GamePad) {
	game_pad.set_button_pressed(Button::A,              callbacks.joypad_button(JoypadButton::A));
	game_pad.set_button_pressed(Button::B,              callbacks.joypad_button(JoypadButton::B));
	game_pad.set_button_pressed(Button::L,              callbacks.joypad_button(JoypadButton::L));
	game_pad.set_button_pressed(Button::R,              callbacks.joypad_button(JoypadButton::R));
	game_pad.set_button_pressed(Button::Start,          callbacks.joypad_button(JoypadButton::Start));
	game_pad.set_button_pressed(Button::Select,         callbacks.joypad_button(JoypadButton::Select));

	game_pad.set_button_pressed(Button::LeftDPadLeft,   callbacks.joypad_button(JoypadButton::Left));
	game_pad.set_button_pressed(Button::LeftDPadRight,  callbacks.joypad_button(JoypadButton::Right));
	game_pad.set_button_pressed(Button::LeftDPadUp,     callbacks.joypad_button(JoypadButton::Up));
	game_pad.set_button_pressed(Button::LeftDPadDown,   callbacks.joypad_button(JoypadButton::Down));

	game_pad.set_button_pressed(Button::RightDPadLeft,  callbacks.joypad_button(JoypadButton::R2));
	game_pad.set_button_pressed(Button::RightDPadRight, callbacks.joypad_button(JoypadButton::R3));
	game_pad.set_button_pressed(Button::RightDPadUp,    callbacks.joypad_button(JoypadButton::L2));
	game_pad.set_button_pressed(Button::RightDPadDown,  callbacks.joypad_button(JoypadButton::L3));
}