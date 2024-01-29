use super::page_table::*;
use crate::config::*;
use core::ops::{Add, Sub};

const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;

const VA_WIDTH_SV39: usize = 39;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SIZE_BITS;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhysAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhysPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
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

impl Add<usize> for PhysAddr {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<usize> for VirtAddr {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<usize> for PhysPageNum {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<usize> for VirtPageNum {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for PhysAddr {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<usize> for VirtAddr {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<usize> for PhysPageNum {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<usize> for VirtPageNum {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl PhysAddr {
    pub fn page_offset(&self) -> usize {
        self.0 & ((1 << PAGE_SIZE_BITS) - 1)
    }

    pub fn page_number_floor(&self) -> PhysPageNum {
        (self.0 >> PAGE_SIZE_BITS).into()
    }

    pub fn page_number_ceil(&self) -> PhysPageNum {
        ((self.0 + (1 << PAGE_SIZE_BITS) - 1) >> PAGE_SIZE_BITS).into()
    }
}

impl VirtAddr {
    pub fn page_offset(&self) -> usize {
        self.0 & ((1 << PAGE_SIZE_BITS) - 1)
    }
}

impl PhysPageNum {
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, PAGE_SIZE / 8) }
    }

    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, PAGE_SIZE) }
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}
