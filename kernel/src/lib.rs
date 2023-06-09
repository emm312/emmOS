#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::panic::PanicInfo;

// extern crate alloc;

use bootloader_api::config::Mapping;
use bootloader_api::BootloaderConfig;
#[cfg(test)]
use bootloader_api::{entry_point, BootInfo};

pub mod gdt;
pub mod interrupts;
// pub mod memory;
pub mod serial;
pub mod vga_buffer;
// pub mod allocator;
pub mod globals;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

#[cfg(test)]
entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    x86_64::instructions::interrupts::enable();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    crate::halt()
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    init();
    test_main();
    crate::halt()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
