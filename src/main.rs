#![cfg_attr(test, allow(unused_imports))]
// Disable the standard Rust library so that Rust
// code can run on bare metal without an
// underlying operating system.
#![cfg_attr(not(test), no_std)]
// Disable the main entry point because the
// freestanding executable does not have access to
// the Rust runtime and crt0.
#![cfg_attr(not(test), no_main)] 

use core::panic::PanicInfo;
use rustic::println;

// Overwrite the normal entry point (not override
// because the normal entry point is removed and it's
// replaced with the function below). Disable the name
// mangling so the compiler doesn't generate some
// cryptic symbol that the linker won't recognize.
// Tell the compiler that it should use the C calling
// convention for this function (instead of the
// unspecified Rust calling convention). The ! return
// type means that the function is diverging,
// i.e. not allowed to ever return. This is required
// because the entry point is not called by any function,
// but invoked directly by the operating system or bootloader.

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Rustic OS works{}", "!");

    loop {} // For now, fulfill the requirement by looping endlessly...
}

// This is the function that the compiler should
// envoke when a panic occurs. It's defined here
// because the standard library is excluded.
// The PanicInfo parameter contains the file and
// line where the panic happened and the optional
// panic message. The function should never return,
// so it is marked as a diverging function by
// returning the “never” type !
#[cfg(not(test))] // only compile when the test flag is not set
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}