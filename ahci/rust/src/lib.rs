#![no_std]
#![allow(dead_code, unused_assignments, unused_mut)]

pub mod drv_ahci;
pub mod libahci;
pub mod libata;
pub mod platform;

use core::panic::PanicInfo;

// Default panic handler for standalone usage
// When used as a dependency, this can be overridden by the parent crate
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
