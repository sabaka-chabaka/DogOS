use crate::drivers::framebuffer::{Color, FramebufferWriter};
use core::fmt;

pub struct Console {
    writer: FramebufferWriter,
    fg: Color,
    bg: Color,
    col: usize,
    row: usize,
    cols: usize,
    rows: usize,
}

impl Console {
    pub fn new(writer: FramebufferWriter) -> Self {
        let cols = writer.width() / writer.char_width();
        let rows = writer.height() / writer.char_height();

        let mut console = Console {
            writer,
            fg: Color::WHITE,
            bg: Color::BLACK,
            col: 0,
            row: 0,
            cols,
            rows,
        };

        console.clear();
        console
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.fg = fg;
        self.bg = bg;
    }

    pub fn clear(&mut self) {
        self.writer.fill(self.bg);
        self.col = 0;
        self.row = 0;
    }

    pub fn writer_mut(&mut self) -> &mut FramebufferWriter {
        &mut self.writer
    }

    fn clear_row(&mut self, row: usize) {
        let char_height = self.writer.char_height();
        let y = row * char_height;
        let width = self.writer.width();
        self.writer.fill_rect(0, y, width, char_height, self.bg);
    }

    fn newline(&mut self) {
        self.col = 0;
        self.row += 1;

        if self.row >= self.rows {
            self.row = 0;
        }

        self.clear_row(self.row);
    }

    pub fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.newline();
            return;
        }

        if c == '\t' {
            const TAB_STOP: usize = 4;
            let next_col = (self.col / TAB_STOP + 1) * TAB_STOP;

            while self.col < next_col {
                self.write_char(' ');
            }

            return;
        }

        if self.col >= self.cols {
            self.newline();
        }

        let x = self.col * self.writer.char_width();
        let y = self.row * self.writer.char_height();
        self.writer.draw_char(x, y, c, self.fg, self.bg);

        self.col += 1;
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }
}

pub static CONSOLE: spin::Mutex<Option<Console>> = spin::Mutex::new(None);

pub fn init(console: Console) {
    *CONSOLE.lock() = Some(console);
}

pub fn with<F: FnOnce(&mut Console)>(f: F) {
    if let Some(console) = CONSOLE.lock().as_mut() {
        f(console);
    }
}

pub unsafe fn force_unlock() {
    CONSOLE.force_unlock();
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    with(|console| {
        let _ = console.write_fmt(args);
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}