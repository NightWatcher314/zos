#![no_std]
#![no_main]
#![feature(panic_info_message)]
#[macro_use]
mod console;
mod lang_item;
pub mod loader;
mod sbi;
mod sync;
mod syscall;
mod task;
mod trap;
use core::arch::global_asm;

global_asm!(include_str!("./entry.asm"));
global_asm!(include_str!("./link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    trap::init();
    loader::init();
    loader::run_next_app();
    // test_code();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe {
        (a as *mut u8).write_volatile(0);
    });
}
