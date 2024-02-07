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

pub trait StepByOne {
    fn step(&mut self);
}
impl StepByOne for VirtPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}

use core::fmt::Debug;
#[derive(Copy, Clone)]
/// a simple range structure for type T
pub struct SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    l: T,
    r: T,
}
impl<T> SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "start {:?} > end {:?}!", start, end);
        Self { l: start, r: end }
    }
    pub fn get_start(&self) -> T {
        self.l
    }
    pub fn get_end(&self) -> T {
        self.r
    }
}
impl<T> IntoIterator for SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;
    type IntoIter = SimpleRangeIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator::new(self.l, self.r)
    }
}
/// iterator for the simple range structure
pub struct SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    current: T,
    end: T,
}
impl<T> SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(l: T, r: T) -> Self {
        Self { current: l, end: r }
    }
}
impl<T> Iterator for SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}

/// a simple range structure for virtual page number
pub type VPNRange = SimpleRange<VirtPageNum>;
