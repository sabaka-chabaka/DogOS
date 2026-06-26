#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA: *mut u8 = 0xb8000 as *mut u8;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        *VGA.offset(0) = b'H';
        *VGA.offset(1) = 0x0F;

        *VGA.offset(2) = b'i';
        *VGA.offset(3) = 0x0F;
    }

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}