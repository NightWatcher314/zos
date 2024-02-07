#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#[macro_use]
mod console;
#[macro_use]
extern crate bitflags;
mod config;
mod lang_item;
pub mod loader;
mod mm;
mod sbi;
mod syscall;
mod task;
mod timer;
mod trap;
mod utils;
use core::arch::global_asm;

use crate::sbi::shutdown;
extern crate alloc;

global_asm!(include_str!("./entry.asm"));
global_asm!(include_str!("./link_app.S"));

#[no_mangle]
pub fn rust_main() {
    clear_bss();
    trap::init();
    print!("heap init start\n");
    mm::init_heap();
    print!("heap init end\n");
    mm::heap_test();
    mm::init_frame_allocator();
    mm::frame_allocator_test();
    shutdown(false);
    // loader::init();
    // trap::enable_timer_interrupt();
    // timer::set_next_trigger();
    // task::run_first_task();
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
