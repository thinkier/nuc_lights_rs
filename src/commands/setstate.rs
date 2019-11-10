use linux_kernel_module::bindings;
use linux_kernel_module::bindings::acpi_buffer;

use linux_kernel_module::println;

use super::*;
use crate::commands::pointer::Pointer;
use core::mem::size_of_val;

pub struct SetState {
	pub buf: acpi_buffer
}

impl SetState {
	pub fn new(led: Led, brightness: u8, blink: Blink, colour: Colour) -> Option<Self> {
		if brightness > 100 {
			return None;
		}

		let ccode = colour.to_code(&led)?;
		let mut buf = Pointer([led as u8, brightness, blink as u8, ccode as u8]);

		Some(SetState {
			buf: acpi_buffer {
				length: unsafe { size_of_val(&buf) } as u64,
				pointer: buf.as_ptr(),
			}
		})
	}

	pub fn apply(&mut self) -> bool {
		let mut res = SetStateResult::new();

		let _acpi_status = unsafe {
			bindings::wmi_evaluate_method(GUID, 0, 0x2, &mut self.buf, &mut res.buf)
		};

		for (k, v) in &res.as_err_strs() {
			println!("{}: {}", k, v);
		}

		return res.is_err();
	}
}

pub struct SetStateResult {
	pub buf: acpi_buffer
}

impl SetStateResult {
	pub fn new() -> Self {
		let mut buf = Pointer([0u32; 3]);

		SetStateResult {
			buf: acpi_buffer {
				length: unsafe { size_of_val(&buf) } as u64,
				pointer: buf.as_ptr(),
			}
		}
	}

	pub fn is_err(&self) -> bool {
		for i in 0..=2 {
			if self.as_slice()[i] != 0 {
				return false;
			}
		}
		return true;
	}

	pub fn as_slice(&self) -> &[u32; 3] {
		unsafe {
			return &*self.buf.pointer.cast();
		}
	}

	pub fn as_err_strs(&self) -> [(&'static str, &'static str); 3] {
		let errs = self.as_slice();
		let mut catalogue = [
			("LED Brightness", "unknown value"),
			("LED Blinking/Fade", "unknown value"),
			("LED Colour", "unknown value")
		];

		for i in 0..=2 {
			catalogue[i].1 = Self::error_byte_to_str(errs[i]);
		}

		return catalogue;
	}

	fn error_byte_to_str(err: u32) -> &'static str {
		match err {
			0x00 => "No Error",
			0xE1 => "Function not supported",
			0xE2 => "Undefined device",
			0xE3 => "EC no response",
			0xE4 => "Invalid parameter",
			0xEF => "Unexpected error",
			_ => "(Reserved Code)",
		}
	}
}
