#![allow(dead_code)]

pub struct Serial {
  port: u16
}

impl Serial {
  pub fn new(port: u16) -> Serial {
    Serial { port: port }
  }

  pub fn outb(&self, byte: u8) {
    outport_b(byte, self.port);
  }

  pub fn inb(&self) -> u8 {
    inport_b(self.port)
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

fn outport_b(byte: u8, port: u16) {
  unsafe {
    asm!("outb %al, %dx" :: "{dx}" (port), "{al}" (byte) :: "volatile");
  }
}

fn inport_b(port: u16) -> u8 {
  let result: u8;
  unsafe {
    asm!("inb %dx, %al" : "={al}" (result) : "{dx}" (port) :: "volatile");
  }
  result
}
