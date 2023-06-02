#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use emmos::println;
use emmos::serial_print;
use emmos::serial_println;

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
pub extern "C" fn _start() -> ! {
    emmos::init();

    //fn stack_overflow() {
    //    stack_overflow(); // for each recursion, the return address is pushed
    //}
    //stack_overflow();

    #[cfg(test)]
    test_main();
    println!("it workr");
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
