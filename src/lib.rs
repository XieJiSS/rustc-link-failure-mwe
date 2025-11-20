pub mod cpp;

pub fn print_int(i: i32) {
    cpp::ffi::print_int(i);
}
