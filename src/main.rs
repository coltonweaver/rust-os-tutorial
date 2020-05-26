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
    
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
    
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rust_os_tutorial::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    rust_os_tutorial::hlt_loop();
}

// Panic Handler Specific to the tests
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rust_os_tutorial::test_panic_handler(_info);
}
