// Disable static linking of the Rust standard library, Rust runtime and crt0; instead we will define our own entrypoint: _start.
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

/// The entrypoint of the kernel.
/// # Naming
/// no_mangle is used to ensure that the name of this function is not changed. Otherwise, Rust would
/// convert this name into a random symbol so each function has a unique name.
/// # Calling Convention
/// This function also uses C calling convention instead of Rust's.
/// # Return Value
/// This function is also divergent and thus will not return, since it is called directly by the Bootloader and not any function.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    
    println!("Hello world!!!!");
    panic!("oh no.");
}
