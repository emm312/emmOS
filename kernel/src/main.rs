#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// extern crate alloc;

use bootloader_api::config::Mapping;
use bootloader_api::info::FrameBufferInfo;
use bootloader_api::BootloaderConfig;
use bootloader_api::{entry_point, BootInfo};
use core::mem::MaybeUninit;
use spin::Mutex;
// use emmos::allocator;
// use emmos::memory::BootInfoFrameAllocator;
use core::panic::PanicInfo;
// use emmos::memory;
use emmos::globals::*;
use emmos::println;
use emmos::serial_print;
use emmos::serial_println;
use x86_64::structures::paging::Page;
use x86_64::VirtAddr;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    emmos::halt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use emmos::exit_qemu;
    use emmos::QemuExitCode;
    serial_println!("[failed]\nError: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    emmos::halt();
}

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    emmos::init();
    let info = boot_info.framebuffer.as_ref().unwrap().info().clone();
    let framebuffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    *FRAMEBUFFER.lock() = MaybeUninit::new(framebuffer);
    *FRAMEINFO.lock() = MaybeUninit::new(info);

    fn somefunc() { somefunc() }
    somefunc();

    #[cfg(test)]
    test_main();
    emmos::halt();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    use emmos::exit_qemu;
    use emmos::QemuExitCode;
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
