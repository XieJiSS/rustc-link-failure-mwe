#![allow(clippy::missing_safety_doc)]

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("example.h");

        fn print_int(i: i32);
    }
}
