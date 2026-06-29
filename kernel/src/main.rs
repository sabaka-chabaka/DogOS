#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod console;
mod cpu;
mod drivers;
mod logger;

use bootloader_api::BootInfo;
use console::Console;
use core::panic::PanicInfo;
use drivers::framebuffer::{Color, FramebufferWriter};
use drivers::serial;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let writer = FramebufferWriter::new(framebuffer);
    console::init(Console::new(writer));

    serial::init();
    logger::init();
    cpu::gdt::init();
    cpu::idt::init();

    log::info!("logger ready, this line goes to screen and serial");

    x86_64::instructions::interrupts::int3();

    log::info!("breakpoint handler returned, execution continues normally");

    console::with(|c| c.set_color(Color::WHITE, Color::BLACK));
    for i in 1..41 {
        println!("log line {}", i);
    }

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        console::force_unlock();
        serial::force_unlock();
    }

    log::error!("kernel panic: {}", info);

    loop {}
}