pub const USER_STACK_SIZE: usize = 4096;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const MAX_APP_NUM: usize = 4;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

pub const CLOCK_FREQ: usize = 12500000;
pub const TICKS_PER_SEC: usize = 100;

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
pub const KERNEL_HEAP_ORDER: usize = 32;

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 12;
