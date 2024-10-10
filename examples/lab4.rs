// #![no_std]
// #![no_main]

// // This code is here to handle panic errors
// use core::panic::PanicInfo;
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

// // do not change this code, only nesseasry for opt-level = 0
// // #[no_mangle]
// // pub extern "C" fn memset(s: *mut u8, c: u8, n: usize) -> *mut u8 {
// //     let ptr = s as *const _ as *mut u8;
// //     let slice = unsafe { core::slice::from_raw_parts_mut(ptr, n) };
// //     slice.fill(c);
// //     slice as *const _ as *mut u8
// // }

// // Allocate memory to write plain text
// #[no_mangle]
// #[export_name = "plain"]
// static mut PLAIN: [?; ?] = [?; ?];

// // Enter your coded data
// #[link_section = ".data"]
// static CODED: [?; ?] = [?];


// #[no_mangle]
// #[link_section = ".text"]
// fn main() -> ! {
//     // seed = ?
//     // initialize plain and let plain point to the allocated memory PLAIN

//     decode(?);
// }

// fn codgen(?) -> ? {}

// fn decode(?) -> ? {}
