#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    os::init();

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn should_trigger_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}