#![no_std]
#![no_main]

mod console;
mod drivers;
mod logger;

use bootloader_api::BootInfo;
use console::Console;
use core::panic::PanicInfo;
use drivers::framebuffer::{Color, FramebufferWriter};

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let writer = FramebufferWriter::new(framebuffer);
    console::init(Console::new(writer));

    drivers::serial::init();
    logger::init();

    println!("DogOS console online");
    println!("Tabs:\tone\ttwo\tthree");

    log::info!("logger ready, this line goes to screen and serial");
    log::warn!("this is a warning, shown in yellow");

    console::with(|c| c.set_color(Color::WHITE, Color::BLACK));
    for i in 0..40 {
        println!("log line {}", i);
    }

    loop {}
}
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        console::force_unlock();
        drivers::serial::force_unlock();
    }

    log::error!("kernel panic: {}", info);

    loop {}
}