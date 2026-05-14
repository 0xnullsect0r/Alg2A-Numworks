// Storage library bindings
#[cfg(target_os = "none")]
pub mod storage {
    use core::ffi::{c_char, c_int};

    unsafe extern "C" {
        pub fn storage_record_name_is_equal_to(name: *const c_char, base_name: *const c_char, extension: *const c_char) -> bool;
        pub fn storage_record_size(name: *const c_char) -> usize;
        pub fn storage_record_exists(name: *const c_char) -> bool;
        pub fn storage_record_read(name: *const c_char, buffer: *mut u8, buffer_size: usize) -> c_int;
        pub fn storage_record_write(name: *const c_char, data: *const u8, size: usize) -> c_int;
        pub fn storage_record_destroy(name: *const c_char);
    }
}
