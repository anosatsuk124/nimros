#![no_std]

use core::slice::from_raw_parts_mut;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum PixelFormat {
    PixelRGBResv8bitPerColor,
    PixelBGRResv8bitPerColor,
}

#[repr(C)]
pub struct FrameBufferConfig {
    pub frame_buffer: *mut u8,
    pub pixels_per_scan_line: u64,
    pub h_resolution: u64,
    pub v_resolution: u64,
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

pub struct FrameBuffer<'a> {
    pub frame_buffer: &'a mut [u8],
    pub pixels_per_scan_line: u64,
    pub h_resolution: u64,
    pub v_resolution: u64,
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
    pub fn write_pixel(&mut self, x: u64, y: u64, pixel_color: &PixelColor) {
        let pixel_position = self.pixels_per_scan_line * y + x;
        let base = (4 * pixel_position) as usize;
        match self.pixel_format {
            PixelFormat::PixelRGBResv8bitPerColor => {
                self.frame_buffer[base + 0] = pixel_color.r;
                self.frame_buffer[base + 1] = pixel_color.g;
                self.frame_buffer[base + 2] = pixel_color.b;
            }
            PixelFormat::PixelBGRResv8bitPerColor => {
                self.frame_buffer[base + 0] = pixel_color.b;
                self.frame_buffer[base + 1] = pixel_color.g;
                self.frame_buffer[base + 2] = pixel_color.r;
            }
        }
    }
}
