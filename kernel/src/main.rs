#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let vga = 0xb8000 as *mut u8;

    let msg = b"DogOS is alive";

    for (i, &c) in msg.iter().enumerate() {
        unsafe {
            *vga.add(i * 2) = c;
            *vga.add(i * 2 + 1) = 0x0f;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}