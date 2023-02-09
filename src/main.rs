#![no_std]

pub mod kos;

#[no_mangle]
pub extern "C" fn main() {
    let pvr_params = kos::pvr::PvrInitParams::new()
        .with_bin_size(kos::pvr::PvrBin::OPAQUE, kos::pvr::PvrBinSize::BINSIZE_16)
        .with_vertex_size(512);

    //kos::pvr::Pvr::init_defaults();
    let init_result = kos::pvr::Pvr::init(pvr_params);

    match init_result {
        kos::common::KosResult::Ok => {
            unsafe {
                kos::c_raw::println("Initialized PVR successfully!\n");
            }
        },
        kos::common::KosResult::Error(e) => {
            unsafe {
                kos::c_raw::println("Error: %s\n", e);
            }
        }
    }
}
