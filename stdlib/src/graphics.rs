use crate::fonts::{Font, FONT_SIZE, HANKAKU_FONT};
use core::{mem, slice::from_raw_parts_mut};

#[repr(C)]
#[derive(Clone, Copy)]
pub enum PixelFormat {
    PixelRGBResv8bitPerColor,
    PixelBGRResv8bitPerColor,
}

#[repr(C)]
pub struct FrameBufferConfig {
    pub frame_buffer: *mut u8,
    pub pixels_per_scan_line: usize,
    pub h_resolution: usize,
    pub v_resolution: usize,
    pub pixel_format: PixelFormat,
    pub size: u64,
}

impl<'a> From<&FrameBufferConfig> for FrameBuffer<'a> {
    fn from(value: &FrameBufferConfig) -> Self {
        let frame_buffer = unsafe { from_raw_parts_mut(value.frame_buffer, value.size as usize) };

        Self {
            frame_buffer,
            pixels_per_scan_line: value.pixels_per_scan_line,
            h_resolution: value.h_resolution,
            v_resolution: value.v_resolution,
            pixel_format: value.pixel_format,
        }
    }
}

#[repr(C)]
pub struct FrameBuffer<'a> {
    pub frame_buffer: &'a mut [u8],
    pub pixels_per_scan_line: usize,
    pub h_resolution: usize,
    pub v_resolution: usize,
    pub pixel_format: PixelFormat,
}

pub struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

impl PixelColor {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn bgr(b: u8, g: u8, r: u8) -> Self {
        Self { r, g, b }
    }
}

impl<'a> FrameBuffer<'a> {
    pub fn write_pixel(&mut self, pos: (usize, usize), pixel_color: &PixelColor) {
        let x = pos.0;
        let y = pos.1;
        let pixel_position = self.pixels_per_scan_line * y + x;
        let base = (4 * pixel_position) as usize;
        let fb: &mut [u8] = self.frame_buffer;
        #[allow(clippy::identity_op)]
        match self.pixel_format {
            PixelFormat::PixelRGBResv8bitPerColor => {
                fb[base + 0] = pixel_color.r;
                fb[base + 1] = pixel_color.g;
                fb[base + 2] = pixel_color.b;
            }
            PixelFormat::PixelBGRResv8bitPerColor => {
                fb[base + 0] = pixel_color.b;
                fb[base + 1] = pixel_color.g;
                fb[base + 2] = pixel_color.r;
            }
        }
    }

    pub fn write_ascii(&mut self, pos: (usize, usize), char: u8, pixel_color: &PixelColor) {
        let font = match HANKAKU_FONT.get_font(char) {
            Ok(font) => font,
            Err(_) => return,
        };

        let font_y_size = 16;
        let font_x_size = 8;
        let x = pos.0;
        let y = pos.1;
        for dy in 0..font_y_size {
            for dx in 0..font_x_size {
                if (font[dy] << dx) & 0x80 != 0 {
                    self.write_pixel((x + dx, y + dy), pixel_color);
                }
            }
        }
    }
}
