#![no_std]
#![no_main]
#[macro_use]
 extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    let p = 5u64;
    let m = 998244353u64;
    let iter: usize = 200000;
    let mut s = [0u64; 100];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == 100 { 0 } else { cur + 1 };
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            println!("power_5 [{}/{}]", i, iter);
        }
    }
    println!("Test power_5 OK!");
    0
}
