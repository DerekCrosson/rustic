// Disable the standard Rust library so that Rust
// code can run on bare metal without an
// underlying operating system.
#![cfg_attr(not(test), no_std)]

pub mod vga_buffer;
pub mod serial;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}