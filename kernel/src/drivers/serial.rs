use spin::Mutex;
use uart_16550::SerialPort;

const COM1: u16 = 0x3F8;

pub static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe {SerialPort::new(COM1)});

pub fn init() {
    SERIAL1.lock().init();
}

pub unsafe fn force_unlock() {
    SERIAL1.force_unlock();
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;

    let _ = SERIAL1.lock().write_fmt(args);
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}