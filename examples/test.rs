#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link_section = ".data"]
static mut DATA_SECTION: [u8; 100] = [0; 100];

#[no_mangle]
pub extern "C" fn memset(s: *mut u8, c: u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe {
            *s.add(i) = c;
        }
        i += 1;
    }
    s
}



#[link_section = ".text"]
#[no_mangle]
fn main() -> ! {
    let mut i = 0;
    while i < 100 {
        unsafe {
            DATA_SECTION[i] = (i + 1) as u8;
        }
        i += 1;
    }

    #[allow(clippy::empty_loop)]
    loop {}
}