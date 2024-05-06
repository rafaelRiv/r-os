// uart.rs
// UART routines and driver

use core::convert::TryInto;
use core::fmt::Write;
use core::fmt::Error;

pub struct Uart {
	base_address: usize,
}

impl Write for Uart {
	fn write_str(&mut self, out: &str) -> Result<(), Error> {
		for c in out.bytes() {
			self.put(c);
		}
		Ok(())
	}
}

impl Uart {
	pub fn new(base_address: usize) -> Self {
		Uart {
			base_address
		}
	}

	pub fn init(&mut self) {
		let ptr = self.base_address as *mut u8;
		unsafe {
			ptr.add(3).write_volatile((1 << 0) | (1 << 1));
			ptr.add(2).write_volatile(1 << 0);
			let divisor: u16 = 592;
			let divisor_least: u8 = (divisor & 0xff).try_into().unwrap();
			let divisor_most:  u8 = (divisor >> 8).try_into().unwrap();
			let lcr = ptr.add(3).read_volatile();
			ptr.add(3).write_volatile(lcr | 1 << 7);
			ptr.add(0).write_volatile(divisor_least);
			ptr.add(1).write_volatile(divisor_most);
			ptr.add(3).write_volatile(lcr);
		}
	}

	pub fn put(&mut self, c: u8) {
		let ptr = self.base_address as *mut u8;
		unsafe {
			ptr.add(0).write_volatile(c);
		}
	}

	pub fn get(&mut self) -> Option<u8> {
		let ptr = self.base_address as *mut u8;
		unsafe {
			if ptr.add(5).read_volatile() & 1 == 0 {
				// The DR bit is 0, meaning no data
				None
			}
			else {
				// The DR bit is 1, meaning data!
				Some(ptr.add(0).read_volatile())
			}
		}
	}
}
