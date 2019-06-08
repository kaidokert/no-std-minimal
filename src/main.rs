#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn handle_panic(_: &PanicInfo) -> ! {
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
    loop {}
}

#[no_mangle]
extern "C" fn rust_eh_personality() {}

extern crate heapless;
use heapless::consts::U8;
use heapless::Vec;

#[no_mangle]
pub extern "C" fn main() {
    let mut xs: Vec<u8, U8> = Vec::new();
    xs.push(42).unwrap();
    let val = xs.pop();
    assert_eq!(val, Some(41));
}
