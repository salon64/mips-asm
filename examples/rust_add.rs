#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut VAR: u32 = 4;

#[link_section = ".text"]
#[no_mangle]
fn main() {
    let mut ptr = unsafe { &mut VAR as *mut _ as *mut u32 };
    loop {
        unsafe { core::ptr::write_volatile(ptr, VAR + 1) };
    }
}
