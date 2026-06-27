#![no_std]
#![no_main]

mod console;
mod drivers;

use bootloader_api::BootInfo;
use console::Console;
use core::fmt::Write;
use core::panic::PanicInfo;
use drivers::framebuffer::{Color, FramebufferWriter};

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let writer = FramebufferWriter::new(framebuffer);
    let mut console = Console::new(writer);

    console.set_color(Color::GREEN, Color::BLACK);
    writeln!(console, "DogOS console online").unwrap();

    for i in 0..80 {
        writeln!(console, "log line {}", i).unwrap();
    }

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}