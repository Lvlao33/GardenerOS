#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate bitflags;

mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod loader;
mod config;
mod task;
mod timer;
mod mm;
mod sync;



use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as *const () as usize as *mut u8,
            ebss as *const () as usize - sbss as *const () as usize,
        ).fill(0);
    }
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[Kernel] Hello, world!");

    mm::init();
    println!("[kernel] back to world!");
    mm::remap_test();
    trap::init();    
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();

    panic!("Unreachable in rust_main!");

}


