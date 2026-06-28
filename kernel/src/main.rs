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
    console::init(Console::new(writer));

    console::with(|console| {
        console.set_color(Color::GREEN, Color::BLACK);
        let _ = writeln!(console, "DogOS console online");

        for i in 0..80 {
            let _ = writeln!(console, "log line {}", i);
        }
    });

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        console::force_unlock();
    }

    console::with(|console| {
        console.set_color(Color::RED, Color::BLACK);
        let _ = writeln!(console, "kernel panic:");
        let _ = writeln!(console, "{}", info);
    });

    loop {}
}