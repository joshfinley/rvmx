#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This is the function thats gets called on boot
#[unsafe(no_mangle)]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    let test_buffer = core::ptr::null_mut::<u8>();

    unsafe {
        *test_buffer = 1;
        *test_buffer.offset(1) = 1;
    }

    loop {}
}

#[panic_handler]
#[allow(clippy::empty_loop)]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
