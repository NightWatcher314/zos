use bitflags::*;

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;// valid
        const R = 1 << 1;// readable
        const W = 1 << 2;// writable
        const X = 1 << 3;// executable
        const U = 1 << 4;// user
        const G = 1 << 5;// global
        const A = 1 << 6;// accessed
        const D = 1 << 7;// dirty
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}
