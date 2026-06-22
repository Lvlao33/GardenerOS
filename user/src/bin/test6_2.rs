#![no_std]
#![no_main]

use user_lib::{sys_write, yield_};

const MSG: &[u8] = b"Second App: Independent Address Space\n";

#[unsafe(no_mangle)]
fn main() -> i32 {
    loop {
        sys_write(1, MSG);
        yield_();
    }
}
