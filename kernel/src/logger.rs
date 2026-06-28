use crate::console;
use crate::drivers::framebuffer::Color;
use core::fmt::Write;
use log::{Level, LevelFilter, Log, Metadata, Record};

struct DogLogger;

impl Log for DogLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let level = record.level();

        let color = match level {
            Level::Error => Color::RED,
            Level::Warn => Color::YELLOW,
            Level::Info => Color::GREEN,
            Level::Debug => Color::CYAN,
            Level::Trace => Color::GRAY,
        };

        console::with(|c| {
            c.set_color(color, Color::BLACK);
            let _ = writeln!(c, "[{}] {}", level, record.args());
        });

        let _ = writeln!(crate::drivers::serial::SERIAL1.lock(), "[{}] {}", level, record.args());
    }

    fn flush(&self) {}
}

static LOGGER: DogLogger = DogLogger;

pub fn init() {
    log::set_logger(&LOGGER).expect("logger already initialized");
    log::set_max_level(LevelFilter::Trace);
}