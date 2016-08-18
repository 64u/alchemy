#![allow(dead_code)]

const HEIGHT: usize = 25;
const WIDTH: usize = 80;

pub enum Color {
  Black      = 0,
  Blue       = 1,
  Green      = 2,
  Cyan       = 3,
  Red        = 4,
  Magenta    = 5,
  Brown      = 6,
  LightGray  = 7,
  DarkGray   = 8,
  LightBlue  = 9,
  LightGreen = 10,
  LightCyan  = 11,
  LightRed   = 12,
  Pink       = 13,
  Yellow     = 14,
  White      = 15
}

#[derive(Clone)]
pub struct ColorCell(u8);

impl ColorCell {
  pub fn new(fg: Color, bg: Color) -> ColorCell {
    ColorCell((bg as u8) << 4 | (fg as u8))
  }
}

#[derive(Clone)]
pub struct VGAChar {
  pub chr:   u8,
  pub color: ColorCell
}

impl VGAChar {
  pub fn write(&self, x: usize, y: usize) {
    unsafe {
      let vga = 0xb8000 as *mut VGAChar;
      *vga.offset((x + (y * WIDTH)) as isize) = self.clone()
    }
  }
}
