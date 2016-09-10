#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![feature(start)]

extern crate rlibc;

// Okay we have to do this to get the documentation for this module
// to be included. Should be fine.
#[macro_use] pub mod kernel;

use kernel::vga::*;
use kernel::port::Serial;

use core::fmt::Write;

#[no_mangle]
pub extern fn main(_argc: isize, _argv: *const *const u8) -> isize {
  let mut console = Serial::new(0x3F8);

  // Simply a drawing test for both VGA and the serial port.
  // Note that wrapping is handled correctly on VGA.
  {
    for i in 0..30 {
      for _ in 0..81 {
        print!("-");
        write!(console, "-").unwrap();
      }

      print!("{}\n", i);
      write!(console, "{}\n", i).unwrap();
    }
  }
  
  write!(VGA.lock(), "read byte: {}\n", console.inb()).unwrap();
  
  loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
  loop {}
}
