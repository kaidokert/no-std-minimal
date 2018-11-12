#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn handle_panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, _src: *const u8, _n: usize) -> *mut u8 {
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, _src: *const u8, _n: usize) -> *mut u8 {
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(mem: *mut u8, _val: i32, _n: usize) -> *mut u8 {
    mem
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(_mem1: *const u8, _mem2: *const u8, _n: usize) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    main()
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    panic!()
}
