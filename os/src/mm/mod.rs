mod heap_allocator;
mod page_table;
mod address;
mod frame_allocator;

mod memory_set;

use crate::{print,println};

use crate::config::MEMORY_END;

use page_table::{PageTable,PTEFlags};
use address::{VPNRange, StepByOne};

pub use page_table::{PageTableEntry, translated_byte_buffer};

pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum};
pub use frame_allocator::{FrameTracker, frame_alloc};

pub use memory_set::{
    MapType, MapPermission, MapArea,
    MemorySet, KERNEL_SPACE, remap_test
};

unsafe extern "C" {
    fn ekernel();
}

pub fn init() {
//    heap_allocator::init_heap();
//    frame_allocator::init_frame_allocator();
//    KERNEL_SPACE.exclusive_access().activate();
    println!("[DEBUG] === mm_init start ===");
    println!("[DEBUG] 3/4 start init frame allocator");
    frame_allocator::init_frame_allocator();
    println!("[DEBUG] 3/4 frame allocator init done");

    println!("[DEBUG] 1/4 init heap allocator");
    heap_allocator::init_heap();
    println!("[DEBUG] 1/4 heap init done");

    println!("[DEBUG] 1/4 run heap test");
    heap_allocator::heap_test();
    println!("[DEBUG] 1/4 heap test passed");

//    println!("[DEBUG] 2/4 check memory config");
//    let ekernel_addr = unsafe { ekernel as usize };
//    println!("[DEBUG] ekernel addr = {:#x}", ekernel_addr);
//    println!("[DEBUG] MEMORY_END = {:#x}", MEMORY_END);

    

    println!("[DEBUG] 3/4 run frame test");
    frame_allocator::frame_allocator_test();
    println!("[DEBUG] 3/4 frame test passed");

    println!("[DEBUG] 4/4 activate MMU");
    KERNEL_SPACE.exclusive_access().activate();
    println!("[DEBUG] 4/4 MMU activated successfully");

    println!("[DEBUG] === mm_init all finished ===");
}


