use super::{address::*, page_table::PageTable};
use crate::mm::frame_allocator::FrameTracker;
use alloc::{collections::BTreeMap, vec::Vec};

pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}
