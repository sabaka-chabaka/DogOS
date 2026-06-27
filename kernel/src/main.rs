#![no_std]
#![no_main]

mod drivers;

use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use drivers::framebuffer::{Color, FramebufferWriter};

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let mut writer = FramebufferWriter::new(framebuffer);

    writer.clear();
    writer.draw_str(20, 20, "DogOS", Color::GREEN, Color::BLACK);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}