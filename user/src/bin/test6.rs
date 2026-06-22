#![no_std]
#![no_main]

use user_lib::{sys_write, yield_};

const BUF: &[u8] = b"Test App6: Virtual Address Space Test\n";

#[unsafe(no_mangle)]
fn main() -> i32 {
    loop {
        sys_write(1, BUF);
        yield_(); 
    }
}
