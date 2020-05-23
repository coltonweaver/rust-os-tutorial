#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_tutorial::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os_tutorial::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    rust_os_tutorial::init();
    
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}

// Panic Handler Specific to the tests
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rust_os_tutorial::test_panic_handler(_info);
}