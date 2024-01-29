use crate::config::*;

const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;

const VA_WIDTH_SV39: usize = 39;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

impl From<usize> for PhysAddr {
    fn from(x: usize) -> Self {
        Self(x & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhysPageNum {
    fn from(x: usize) -> Self {
        Self(x & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtAddr {
    fn from(x: usize) -> Self {
        Self(x & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for VirtPageNum {
    fn from(x: usize) -> Self {
        Self(x & ((1 << VPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysAddr> for usize {
    fn from(x: PhysAddr) -> Self {
        x.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(x: PhysPageNum) -> Self {
        x.0
    }
}

impl From<VirtAddr> for usize {
    fn from(x: VirtAddr) -> Self {
        x.0
    }
}

impl From<VirtPageNum> for usize {
    fn from(x: VirtPageNum) -> Self {
        x.0
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(x: PhysAddr) -> Self {
        assert_eq!(x.page_offset(), 0);
        Self(x.0 >> PAGE_SIZE_BITS)
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(x: VirtAddr) -> Self {
        assert_eq!(x.page_offset(), 0);
        Self(x.0 >> PAGE_SIZE_BITS)
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(x: PhysPageNum) -> Self {
        Self(x.0 << PAGE_SIZE_BITS)
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(x: VirtPageNum) -> Self {
        Self(x.0 << PAGE_SIZE_BITS)
    }
}

impl PhysAddr {
    pub fn page_offset(&self) -> usize {
        self.0 & ((1 << PAGE_SIZE_BITS) - 1)
    }
}

impl VirtAddr {
    pub fn page_offset(&self) -> usize {
        self.0 & ((1 << PAGE_SIZE_BITS) - 1)
    }
}
