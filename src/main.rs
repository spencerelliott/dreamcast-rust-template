#![no_std]

mod kos;
use kos::Kos;

#[no_mangle]
pub extern "C" fn main() {
    let kos: Kos = Kos {};
    kos.test();
}
