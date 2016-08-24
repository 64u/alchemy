#![allow(dead_code)]

pub struct Serial {
  port: u16
}

impl Serial {
  pub fn new(port: u16) -> Serial {
    Serial { port: port }
  }

  pub fn outb(&self, byte: u8) {
    unsafe { outport_b(byte, self.port) };
  }

  pub fn inb(&self) -> u8 {
    unsafe { inport_b(self.port) }
  }
}

impl ::core::fmt::Write for Serial {
  fn write_str(&mut self, src: &str) -> ::core::fmt::Result {
    for byte in src.bytes() {
      self.outb(byte);
    }
    
    Ok(())
  }
}

extern {
  fn outport_b(byte: u8, port: u16);
}

extern {
  fn inport_b(port: u16) -> u8;
}
