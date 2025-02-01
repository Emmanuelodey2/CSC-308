#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use spin::mutex::Mutex;
use x86_64::instructions::hlt;
use core::panic::PanicInfo;
use crate::macros::WRITER;
use lazy_static::lazy_static;
#[macro_use] // Brings macros from macros.rs into scope
mod macros; // Import macros module
mod writer; // Import writer module for FrameBufferWriter

use writer::FrameBufferWriter;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
        hlt();
    }
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let frame_buffer_info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    // Initialize WRITER with the framebuffer buffer and frame info
    *WRITER.lock() = Some(FrameBufferWriter::new(buffer, frame_buffer_info));

    // Now you can safely write using the print! and println! macros
    println!("\tHello, Rust OS!");
    println!("Overflow \x16test: line 1");
    println!("Another line to check overflow. This should wrap.");

    loop {
        hlt();
    }
}

entry_point!(kernel_main);
