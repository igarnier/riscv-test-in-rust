#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// defines a _start symbol
global_asm!(include_str!("add.S"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
   loop {}
}

// Main program function
// #[no_mangle]
// extern "C" fn main() -> () {
// //    unsafe { _start(); }
// }
