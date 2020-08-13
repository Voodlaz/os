#![no_std]
#![no_main]

extern crate rlibc;
use core::panic::PanicInfo;

#[no_mangle]
fn _start() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
