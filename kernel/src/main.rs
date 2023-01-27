#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo, slice::from_raw_parts_mut};

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer: (*mut u8, u64)) -> ! {
    let mut fb = unsafe { from_raw_parts_mut(frame_buffer.0, frame_buffer.1 as usize) };
    for i in 0..fb.len() {
        let i = i as usize;
        fb[i] = 255;
    }
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
