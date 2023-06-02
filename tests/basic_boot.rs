#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(emmos::test_runner)]

use core::panic::PanicInfo;
use emmos::{println, serial_println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    serial_println!("basic");
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    emmos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
