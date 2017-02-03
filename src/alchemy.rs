#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![feature(start)]

extern crate rlibc;

// Okay we have to do this to get the documentation for this module
// to be included. Should be fine.
#[macro_use] pub mod kernel;

use kernel::vga::VGA;
use kernel::port::Serial;
use kernel::memory::*;

#[no_mangle]
pub extern fn main(_argc: isize, _argv: *const *const u8) -> isize {
  let mut console = Serial::new(0x3F8);
  let mut memory = Manager::new();
  let mut block = memory.page();

  VGA.lock().clear_screen();

  println!("Block address: {:?}", block);

  loop {}
}

#[lang = "eh_personality"] pub extern fn eh_personality() {}
#[no_mangle] #[lang = "panic_fmt"] pub extern fn panic_fmt() -> ! { loop {} }

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
  loop {}
}
