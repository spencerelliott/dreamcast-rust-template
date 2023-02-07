#![no_std]

pub mod kos;

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        kos::c_raw::println("Hello, Rust!\n");
    }
}
