mod getstate;
mod setstate;
pub mod pointer;

pub use getstate::*;
pub use setstate::*;


use crate::GUID;

pub enum Led {
	Power = 1,
	Ring = 2,
}

pub enum Blink {
	AlwaysOn = 4,
	Blink1s = 1,
	Blink2s = 5,
	Blink4s = 2,
	Fade1s = 3,
	Fade4s = 6,
	Fade2s = 7,
}

#[derive(PartialEq)]
pub enum Colour {
	Disable,
	Blue,
	Amber,
	Cyan,
	Pink,
	Yellow,
	Red,
	Green,
	White,
}

impl Colour {
	pub fn to_code(&self, led: &Led) -> Option<u8> {
		if *self == Colour::Disable {
			return Some(0);
		}

		match *led {
			Led::Power => {
				match *self {
					Colour::Blue => Some(1),
					Colour::Amber => Some(2),
					_ => None
				}
			}
			Led::Ring => {
				match *self {
					Colour::Cyan => Some(1),
					Colour::Pink => Some(2),
					Colour::Yellow => Some(3),
					Colour::Blue => Some(4),
					Colour::Red => Some(5),
					Colour::Green => Some(6),
					Colour::White => Some(7),
					_ => None
				}
			}
		}
	}
}
