#![allow(dead_code)]

/// Primary structure for communicating with serial ports.
pub struct Serial {
  port: u16
}

impl Serial {
  /// Create an instance of `Serial` from a 16-bit address.
  pub fn new(port: u16) -> Serial {
    Serial { port: port }
  }

  /// Write a single byte to the serial port.
  pub fn outb(&self, byte: u8) {
    unsafe { outport_b(byte, self.port) };
  }

  /// Read a single byte from the serial port.
  /// On the event no byte is available, `0` will be returned.
  pub fn inb(&self) -> u8 {
    unsafe { inport_b(self.port) }
  }
}

/// Implementation of Write for the Serial port.
impl ::core::fmt::Write for Serial {
  fn write_str(&mut self, src: &str) -> ::core::fmt::Result {
    for byte in src.bytes() {
      self.outb(byte);
    }
    
    Ok(())
  }
}

/// Externally defined in `port.s`, assembly label 
/// which writes a byte to the given port.
extern {
  fn outport_b(byte: u8, port: u16);
}

/// Externally defined in `port.s`, assembly label
/// which reads a byte from the given port.
extern {
  fn inport_b(port: u16) -> u8;
}