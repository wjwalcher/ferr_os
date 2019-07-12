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
    use core::fmt::Write;

    write!(print_util::WRITER.lock(), "==================== Welcome to ferr_os =================\n");
    write!(print_util::WRITER.lock(), "> BETA VERSION {}.{}.{}\n", 0, 0, 1);
    write!(print_util::WRITER.lock(), "> Architecture: {}\n", "x86_64");
    write!(print_util::WRITER.lock(), "> Initializing...\n");
    
    loop {}
}