#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::get_taskinfo;

#[no_mangle]
fn main() -> i32 {
    let app_id = get_taskinfo();
    println!("Hello, world! taskinfo:{}", app_id);
    0
}
