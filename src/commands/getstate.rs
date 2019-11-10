use linux_kernel_module::bindings;
use linux_kernel_module::bindings::acpi_buffer;

use linux_kernel_module::println;

use super::*;
use crate::commands::pointer::Pointer;
use core::mem::size_of_val;

pub struct GetState {
	pub buf: acpi_buffer
}

impl GetState {
	pub fn new(led: Led) -> Self {
		let mut buf = Pointer([led as u32]);

		GetState {
			buf: acpi_buffer {
				length: unsafe { size_of_val(&buf) } as u64,
				pointer: buf.as_ptr(),
			}
		}
	}

	pub fn get(&mut self) -> GetStateResult {
		let mut res = GetStateResult::new();

		let _acpi_status = unsafe {
			bindings::wmi_evaluate_method(GUID, 0, 0x1, &mut self.buf, &mut res.buf)
		};

		return res;
	}
}

pub struct GetStateResult {
	pub buf: acpi_buffer
}

impl GetStateResult {
	pub fn new() -> Self {
		let mut buf = Pointer([0u32; 4]);

		GetStateResult {
			buf: acpi_buffer {
				length: unsafe { size_of_val(&buf) } as u64,
				pointer: buf.as_ptr(),
			}
		}
	}

	pub fn is_err(&self) -> bool {
		return self.as_slice()[0] != 0;
	}

	pub fn as_slice(&self) -> &[u32; 4] {
		unsafe {
			return &*self.buf.pointer.cast();
		}
	}

	pub fn brightness(&self) -> u32 {
		self.as_slice()[1]
	}

	pub fn blink(&self) -> Blink {
		match self.as_slice()[2] {
			1 => Blink::Blink1s,
			2 => Blink::Blink4s,
			3 => Blink::Fade1s,
			4 => Blink::AlwaysOn,
			5 => Blink::Blink2s,
			6 => Blink::Fade4s,
			7 => Blink::Fade2s,
			_ => Blink::AlwaysOn
		}
	}

	pub fn colour(&self) -> u32 {
		self.as_slice()[3]
	}
}
