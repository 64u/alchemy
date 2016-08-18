#![feature(lang_items)]
#![no_std]
#![feature(start)]
#![feature(libc)]

extern crate libc;

mod vga;
use vga::*;

#[no_mangle]
pub extern fn main(_argc: isize, _argv: *const *const u8) -> isize {
  let a = VGAChar {
    color: ColorCell::new(Color::Red, Color::Black),
    chr: 0x41
  };

  a.write(0, 0);
  a.write(1, 0);
  a.write(0, 1);
  a.write(1, 1);
  
  loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
