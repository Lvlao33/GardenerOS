#![no_std]
#![no_main]

use user_lib::*;

#[unsafe(no_mangle)]
fn main() -> i32 {
    print!("Hello! This is my own app!\n");
    exit(0);
    0
}
