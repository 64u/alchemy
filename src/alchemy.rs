#![feature(lang_items)]
#![no_std]
#![feature(start)]

extern crate rlibc;

mod vga;
use vga::*;

use core::fmt::Write;

#[no_mangle]
pub extern fn main(_argc: isize, _argv: *const *const u8) -> isize {
  let mut writer = Writer::new(ColorPair::new(Color::White, Color::Black));

  write!(writer, "Hello world!").expect("file a bug maybe");
  
  loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
  loop {}
}
