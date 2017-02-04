#![allow(dead_code)]

extern crate spin;

use core::sync::atomic::{AtomicPtr, Ordering};

/// Height of the VGA output in characters.
pub const HEIGHT: usize = 25;
/// Width of the VGA output in characters.
pub const WIDTH: usize = 80;

/// Enumerative type which is used for foreground and backgroun
/// colors alike.
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

/// A pair of `Color`s which represents foreground/background.
/// Note that both colors are packed into a single `u8` to save space.
/// A `VGAChar` is 16-bit and thus the color forms the latter byte. 
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ColorPair(u8);

impl ColorPair {
  /// Create a `ColorPair` from a foreground and background `Color`.
  pub const fn new(fg: Color, bg: Color) -> ColorPair {
    ColorPair((bg as u8) << 4 | (fg as u8))
  }
}

/// This structure represents a single character on the VGA display and its
/// formatting options (only foreground and background color at present).
/// Each character is two bytes, with the first byte containing the ASCII
/// character code and the latter byte containing the formatting options.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VGAChar {
  pub chr:   u8,
  pub color: ColorPair
}

/// `Buffer` is a fixed-size two-dimensional array of `VGAChars` which
/// is sized according to `WIDTH` and `HEIGHT`.
struct Buffer {
  pub chars: [[VGAChar; WIDTH]; HEIGHT]
}

/// `Writer` is a representation of VGA state, it contains positioning
/// information for the current row and column, as well as formatting options
/// and an `AtomicPtr` to a `Buffer`.
pub struct Writer {
  x: usize,
  y: usize,
  pub color: ColorPair,
  buffer: AtomicPtr<Buffer>
}

impl Writer {
  /// Creates a new `Writer` from a `ColorPair`. 
  ///Note that the buffer pointer is hardcoded.
  pub const fn new(color: ColorPair) -> Writer {
    Writer {
      x: 0,
      y: 0,
      color: color,
      buffer: AtomicPtr::new(0xb8000 as *mut _)
    }
  }
  
  /// Writes a single byte to the screen, and takes care to handle 
  /// the edge of the screen and newlines appropriately by updating 
  /// the structure's internal state.
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

  // Handles a newline by updating internal state.
  // Will scroll the screen upwards as necessary.
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

  // Clears a line on the screen.
  fn clear_line(&mut self, line: usize) {
    self.buffer().chars[line] = [VGAChar { chr: b' ', color: self.color }; WIDTH];
  }

  // Clears each line on the screen, iteratively.
  pub fn clear_screen(&mut self) {
    for i in 0..HEIGHT {
      self.clear_line(i);
    }
  }

  // Obtains a mutable reference to the `Buffer` pointer.
  fn buffer(&mut self) -> &mut Buffer {
    unsafe { self.buffer.load(Ordering::Relaxed).as_mut().unwrap() }
  }
}

/// An implementation of `Write` for the `Writer`. 
impl ::core::fmt::Write for Writer {
  fn write_str(&mut self, src: &str) -> ::core::fmt::Result {
    for byte in src.bytes() {
      self.write_byte(byte);
    }

    Ok(())
  }
}

/// A static VGA which makes use of `spin::Mutex` to provide locking.
/// It is this static structure which makes `println!` et al. possible.
pub static VGA: spin::Mutex<Writer> = spin::Mutex::new(
  Writer::new(ColorPair::new(Color::White, Color::Black))
);

macro_rules! print {
  ($($arg:tt)*) => {{
    use core::fmt::Write;
    ($crate::kernel::vga::VGA.lock().write_fmt(format_args!($($arg)*)).unwrap());
  }}
}

macro_rules! println {
  ($fmt:expr) => (print!(concat!($fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
