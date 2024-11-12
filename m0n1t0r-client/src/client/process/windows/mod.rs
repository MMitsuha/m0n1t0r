#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("../m0n1t0r-cpp-windows-lib/include/process.h");
    }
}
