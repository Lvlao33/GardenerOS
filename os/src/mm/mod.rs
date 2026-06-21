mod heap_allocator;
mod page_table;
mod address;
mod frame_allocator;

mod memory_set;

use crate::print;

use page_table::{PageTable,PTEFlags};
use address::{VPNRange, StepByOne};

pub use page_table::{PageTableEntry, translated_byte_buffer};

pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum};
pub use frame_allocator::{FrameTracker, frame_alloc};

pub use memory_set::{
    MapType, MapPermission, MapArea,
    MemorySet, KERNEL_SPACE, remap_test
};


pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();




//    print!("mm::init() start\n");
//    heap_allocator::init_heap();
//    print!("init_heap done\n");
//    heap_allocator::heap_test();
//    print!("init_frame_allocator done\n");
//    frame_allocator::init_frame_allocator();
//    print!("frame_allocator_test done\n"); 
//    frame_allocator::frame_allocator_test();
}


