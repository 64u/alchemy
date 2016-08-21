#![feature(lang_items)]
#![feature(asm)]
#![no_std]
#![feature(start)]

extern crate rlibc;

mod kernel;
use kernel::vga::*;
use kernel::port::Serial;

use core::fmt::Write;

#[no_mangle]
pub extern fn main(_argc: isize, _argv: *const *const u8) -> isize {
  let mut writer = Writer::new(ColorPair::new(Color::White, Color::Black));  
  let mut console = Serial::new(0x3F8);
  
  {
    for i in 0..30 {
      for _ in 0..81 {
        write!(writer, "-").unwrap();
        write!(console, "-").unwrap();
      }
      
      write!(writer, "{}\n", i).unwrap();
      write!(console, "{}\n", i).unwrap();
    }
  }
  
  loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
  loop {}
}
