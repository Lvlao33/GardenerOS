#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;
use user_lib::{get_time, yield_};

#[unsafe(no_mangle)]
fn main() -> i32 {
    let end_time = get_time() + 3000;
    while get_time() < end_time {
        yield_();
    }
    println!("Test sleep OK!");
    0
}
