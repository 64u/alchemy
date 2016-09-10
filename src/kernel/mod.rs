/// This module contains submodules which correspond to various aspects
/// of the kernel such as VGA driver, serial port access...
#[macro_use]

/// This module contains a VGA driver which makes use of a `Mutex` provided
/// by the `spin` crate (spinlock) and an `AtomicPtr` to enable access to
/// an instance of the `Writer` structure statically.
pub mod vga;

/// This module contains some abstractions over external functions
/// which are written in native assembly, contained in 
/// platforms/(arch)/port.s
pub mod port;
