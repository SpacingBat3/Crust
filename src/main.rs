#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info:&PanicInfo) -> ! { unsafe { libc::abort(); } }
#[unsafe(no_mangle)]
fn rust_eh_personality() {}

use libc::{
    printf,
    size_t
};
use core::ffi::*;

#[unsafe(no_mangle)]
unsafe extern "C" fn main(argc:c_int, argv: *const *const c_char)->c_int { unsafe {
    printf(c"Hello world in Crust!\n".as_ptr());
    if argc > 1 {
        printf(c"You can even print your argv:\n".as_ptr());
        for i in (1 as size_t)..(argc as size_t) {
            printf(c" * %s\n".as_ptr(),*argv.add(i));
        }
        printf(c":D\n".as_ptr());
    }
    return 0;
}}
