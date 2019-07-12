#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod print_util;

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
} 

#[no_mangle]
pub extern "C" fn _start() -> ! {

    print_util::kprintln("==================== Welcome to ferr_os =================");
    print_util::kprintln("> BETA VERSION 0.0.1");
    print_util::kprintln("> Architecture: x86_64");
    print_util::kprintln("> Initializing...");
    
    loop {}
}