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
        let rows = writer.height() / writer.height();

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
        let char_height = self.writer.height();
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

        if self.col >= self.cols {
            self.newline();
        }

        let x = self.col * self.writer.char_width();
        let y = self.row * self.writer.height();
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