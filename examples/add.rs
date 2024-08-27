#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
use core::arch::global_asm;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

global_asm! {
    include_str!("add.s")
}
