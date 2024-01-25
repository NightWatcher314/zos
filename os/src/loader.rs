use core::{arch::asm, num};

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

pub fn load_apps() {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = unsafe { num_app_ptr.read_volatile() };
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app) };
    unsafe {
        asm!("fence.i");
    }
    for i in 0..num_app {
        let base_i = get_base_i(i);
        (base_i..base_i + APP_SIZE_LIMIT).for_each(|addr| {
            unsafe { (addr as *mut u8).write_volatile(0) };
        })
    }
    let src = unsafe {
        core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
    };
    let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };
    dst.copy_from_slice(src);
}

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}
