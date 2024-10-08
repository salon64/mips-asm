#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[used]
#[allow(dead_code)]
#[link_section = ".data"]
static mut PLAIN: [u8; 132] = [0; 132];

#[link_section = ".data"]
static CODED: [u32; 86] = [
    0x57aa578d, 0x5bea35dc, 0xfaad75f4, 0x97be0436, 0xc85bc050, 0x03470e2b, 0x2d4db7ff, 0xcd1ec180,
    0x51af0923, 0x3aa1dd7c, 0x9bd17153, 0x46f10c48, 0x5b785b9b, 0x4c4be5dc, 0xa7ffa660, 0xa09b093e,
    0xa8244ea4, 0xb2f6a668, 0xe805b2c8, 0xeac7772e, 0x474a6730, 0x5ed34134, 0x8eb03b48, 0x7d8469cb,
    0x46a199b8, 0xb7d84719, 0xe2c8cac8, 0xaf9c2d6f, 0x14de783c, 0x7b8d7321, 0xfe62aed8, 0x478429ef,
    0x38d8c625, 0x8f7faced, 0x4e2044d3, 0x56285143, 0xc581ba55, 0x29899495, 0xb4c48a07, 0x0ebdc017,
    0x63fb15dd, 0xda1da98a, 0xb0339a29, 0xbf35cc8a, 0x2445d505, 0x1385cc56, 0x4114e805, 0xd36704a2,
    0x969e529d, 0xfc55f842, 0xd3ee536e, 0xd492f1ec, 0x17ed0922, 0xc8756b9e, 0x57f07d66, 0x4c5074ac,
    0x9bb75c7e, 0x3a6ee2da, 0xfc77af3a, 0xb7c3404d, 0xe658045a, 0x15b1d3b3, 0x1f01d14a, 0x6da11831,
    0xb08b0fce, 0xf9d60c53, 0x9d591c0a, 0x930f4fdd, 0xc7231ccb, 0xe627f613, 0xaca7c8f1, 0x3e65d141,
    0x79103ebb, 0x96df302b, 0x4d10f22d, 0x86b88ba5, 0xc53c3137, 0x3856bacc, 0x4c853583, 0x49466f6c,
    0x07209c2f, 0x95bdf638, 0x11ddbeef, 0xb3b85bcc, 0x66e5b797, 0x0,
];

#[no_mangle]
pub extern "C" fn memset(s: *mut u8, c: u8, n: usize) -> *mut u8 {
    let ptr = s as *const _ as *mut u8;
    let slice = unsafe { core::slice::from_raw_parts_mut(ptr, n) };
    slice.fill(c);
    slice as *const _ as *mut u8
}

#[link_section = ".text"]
#[no_mangle]
fn main() -> ! {
    let mut seed = 0x20c99db1;
    let mut plain: [u8; 132] = [0; 132];

    decode(&CODED, &mut plain, &mut seed);

    #[allow(clippy::empty_loop)]

    loop {}
}

#[link_section = ".text"]
fn codgen(seed: &mut u32) -> u32 {
    let mut n: i32 = ((*seed >> 25) & 0x1f) as i32;

    while n >= 0 {
        let x = *seed << 1;
        let y = *seed / 128;
        *seed = x ^ y;
        n -= 1;
    }

    *seed ^ 0x49d38788
}

#[link_section = ".text"]
fn decode(wordarr: &[u32], bytearr: &mut [u8], seed: &mut u32) -> u32 {
    let x = !codgen(seed);

    if wordarr[0] == 0 {
        bytearr[0] = 0;
        x
    } else {
        let y = decode(&wordarr[1..], &mut bytearr[1..], seed);
        let m = (x ^ y).wrapping_sub(wordarr[0]);
        bytearr[0] = (m >> 11) as u8;

        #[allow(clippy::let_and_return)]
        let r = x
            .wrapping_add(y)
            .wrapping_add(m)
            .wrapping_add((-(codgen(seed) as i32)) as u32)
            .wrapping_add(5);

        r
    }
}
