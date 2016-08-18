#![feature(lang_items)]
#![no_std]
#![feature(start)]
#![feature(libc)]

extern crate libc;

#[no_mangle]
pub extern fn main(_argc: isize, _argv: *const *const u8) -> isize {
  0
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
