#![no_std]
#![allow(dead_code, unused_assignments, unused_mut)]

pub mod drv_ahci;
pub mod libahci;
pub mod libata;
pub mod platform;

// Panic handler is only included when building as a standalone library
// When used as a dependency, the parent crate should define the panic handler
#[cfg(not(test))]
#[cfg(feature = "panic-handler")]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
