#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};
use mikanlib::graphics::{FrameBuffer, FrameBufferConfig, PixelColor};

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_config: &FrameBufferConfig) -> ! {
    let mut frame_buffer = FrameBuffer::from(frame_buffer_config);
    let white = PixelColor::rgb(255, 255, 255);
    for x in 0..frame_buffer.h_resolution {
        for y in 0..frame_buffer.v_resolution {
            frame_buffer.write_pixel((x, y), &white);
        }
    }
    let green = PixelColor::rgb(0, 255, 0);
    for x in 0..200 {
        for y in 0..100 {
            frame_buffer.write_pixel((100 + x, 100 + y), &green);
        }
    }
    let black = PixelColor::rgb(0, 0, 0);
    frame_buffer.write_ascii((50, 50), b'A', &black);
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
