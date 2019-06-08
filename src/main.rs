#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn handle_panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() {}
