#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use os::panic_print;
use os::{QemuExitCode, exit_qemu , serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::panic_print!("KERNEL PANIC: {}\n", info);
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}


#[test_case]
fn should_fail() {
    serial_println!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}