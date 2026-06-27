use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use core::cmp::min;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const CYAN: Color = Color::new(0, 255, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 255);
    pub const GRAY: Color = Color::new(128, 128, 128);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

pub struct FramebufferWriter {
    buffer: &'static mut [u8],
    info: FrameBufferInfo,
}

impl FramebufferWriter {
    pub fn new(framebuffer: &'static mut FrameBuffer) -> Self {
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();
        FramebufferWriter { buffer, info }
    }

    pub fn info(&self) -> FrameBufferInfo {
        self.info
    }

    pub fn width(&self) -> usize {
        self.info.width
    }

    pub fn height(&self) -> usize {
        self.info.height
    }

    fn pixel_offset(&self, x: usize, y: usize) -> usize {
        (y * self.info.stride + x) * self.info.bytes_per_pixel
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width() || y >= self.height() {
            return;
        }

        let bytes_per_pixel = self.info.bytes_per_pixel;
        let offset = self.pixel_offset(x, y);
        let pixel = &mut self.buffer[offset..offset + bytes_per_pixel];

        match self.info.pixel_format {
            PixelFormat::Rgb => {
                pixel[0] = color.r;
                pixel[1] = color.g;
                pixel[2] = color.b;
            }
            PixelFormat::Bgr => {
                pixel[0] = color.b;
                pixel[1] = color.g;
                pixel[2] = color.r;
            }
            PixelFormat::U8 => {
                let gray = (color.r as u16 + color.g as u16 + color.b as u16) / 3;
                pixel[0] = gray as u8;
            }
            _ => {}
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        let x_end = min(x.saturating_add(width), self.width());
        let y_end = min(y.saturating_add(height), self.height());

        for py in y..y_end {
            for px in x..x_end {
                self.set_pixel(px, py, color);
            }
        }
    }

    pub fn fill(&mut self, color: Color) {
        let (width, height) = (self.width(), self.height());
        self.fill_rect(0, 0, width, height, color);
    }

    pub fn clear(&mut self) {
        self.fill(Color::BLACK);
    }

    pub fn draw_hline(&mut self, x: usize, y: usize, length: usize, color: Color) {
        self.fill_rect(x, y, length, 1, color);
    }

    pub fn draw_vline(&mut self, x: usize, y: usize, length: usize, color: Color) {
        self.fill_rect(x, y, 1, length, color);
    }

    pub fn draw_rect_outline(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        if width == 0 || height == 0 {
            return;
        }

        self.draw_hline(x, y, width, color);
        self.draw_hline(x, y + height - 1, width, color);
        self.draw_vline(x, y, height, color);
        self.draw_vline(x + width - 1, y, height, color);
    }
}
