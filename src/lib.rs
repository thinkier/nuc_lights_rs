#![no_std]
#![feature(const_transmute)]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;
use core::intrinsics::transmute;

mod commands;

use commands::*;

use linux_kernel_module::println;

pub const GUID: *const i8 = unsafe { transmute(b"8C5DA44C-CDC3-46b3-8619-4E26D34390B7\0".as_ptr()) };


struct NucLights {}

impl linux_kernel_module::KernelModule for NucLights {
	fn init() -> linux_kernel_module::KernelResult<Self> {
		println!("Flicking on the party lights!");
		let mut req = SetState::new(Led::Ring, 100, Blink::Fade2s, Colour::White).unwrap();

		req.apply();

		Ok(NucLights {})
	}
}

impl Drop for NucLights {
	fn drop(&mut self) {
		let mut req = GetState::new(Led::Power);

		let resp = req.get();
		println!("Power LED Brightness: {}", resp.brightness());
		println!("Power LED Blink/fade status: {}", resp.blink() as u8);
		println!("Power LED Colour: {}", resp.colour());

		let mut req = GetState::new(Led::Ring);

		let resp = req.get();
		println!("Ring LED Brightness: {}", resp.brightness());
		println!("Ring LED Blink/fade status: {}", resp.blink() as u8);
		println!("Ring LED Colour: {}", resp.colour());
		println!("Caught shutdown. Exited cleanly.")
	}
}

linux_kernel_module::kernel_module!(
    NucLights,
    // author: "thinkier",
    // description: "Basic Linux Kernel Module to control the lights for the Intel Nuc",
    // license: "GPL"
);

// Temporary hack while FiaB get their implementation in
#[link_section = ".modinfo"]
#[allow(non_upper_case_globals)]
pub static author: [u8; 16] = *b"author=thinkier\0";
#[link_section = ".modinfo"]
#[allow(non_upper_case_globals)]
pub static description: [u8; 78] = *b"description=Basic Linux Kernel Module to control the lights for the Intel NUC\0";
#[link_section = ".modinfo"]
#[allow(non_upper_case_globals)]
pub static license: [u8; 12] = *b"license=GPL\0";