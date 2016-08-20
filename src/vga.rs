#![allow(dead_code)]

pub const HEIGHT: usize = 25;
pub const WIDTH: usize = 80;

#[repr(u8)]
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

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ColorPair(u8);

impl ColorPair {
  pub fn new(fg: Color, bg: Color) -> ColorPair {
    ColorPair((bg as u8) << 4 | (fg as u8))
  }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VGAChar {
  pub chr:   u8,
  pub color: ColorPair
}

struct Buffer {
  pub chars: [[VGAChar; WIDTH]; HEIGHT]
}

pub struct Writer {
  x: usize,
  y: usize,
  pub color: ColorPair,
  buffer: *mut Buffer
}

impl Writer {
  pub fn new(color: ColorPair) -> Writer {
    Writer {
      x: 0,
      y: 0,
      color: color,
      buffer: 0xb8000 as *mut _
    }
  }
  
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      
      byte => {
        if self.x >= WIDTH {
          self.new_line()
        }
        
        self.buffer().chars[self.y][self.x] = VGAChar {
          chr: byte,
          color: self.color
        };

        self.x = self.x + 1
      }
    }
  }

  fn new_line(&mut self) {
    self.x = 0;
    
    if self.y < (HEIGHT - 1) {
      self.y = self.y + 1;
    }

    else {
      self.y = HEIGHT - 1;

      {
        let mut buffer = self.buffer();
        
        for row in 0..(HEIGHT - 1) {
          buffer.chars[row] = buffer.chars[row + 1];
        }
      };

      self.clear_line(HEIGHT - 1);
    }
  }

  fn clear_line(&mut self, line: usize) {
    self.buffer().chars[line] = [VGAChar { chr: b' ', color: self.color }; WIDTH];
  }

  fn clear_screen(&mut self) {
    for i in 0..HEIGHT {
      self.clear_line(i);
    }
  }

  fn buffer(&mut self) -> &mut Buffer {
    unsafe { self.buffer.as_mut().unwrap() }
  }
}

impl ::core::fmt::Write for Writer {
  fn write_str(&mut self, src: &str) -> ::core::fmt::Result {
    for byte in src.bytes() {
      self.write_byte(byte);
    }

    Ok(())
  }
}
