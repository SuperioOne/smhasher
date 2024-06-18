use algorithms::hash::cityhash::hash_fn::{cityhash_64, cityhash_64_with_seed};
use algorithms::hash::{
    crc::hash_fn::crc32c_with_initial, murmur3::hash_fn::murmurhash3_128,
    murmur3::hash_fn::murmurhash3_32,
};
use std::ffi::c_void;
use std::ptr::copy_nonoverlapping;
use std::slice::from_raw_parts;

// FFI wrapper for testing purposes, incorrect length, null ptr cases are ignored.

macro_rules! byte_slice {
    ($ptr:expr, $len:expr) => {{
        unsafe { from_raw_parts($ptr as *const u8, $len) }
    }};
}

macro_rules! copy_to_output {
    ($ptr:expr, $bytes:expr) => {
        unsafe { copy_nonoverlapping($bytes.as_ptr(), $ptr as *mut u8, $bytes.len()) };
    };
}

#[no_mangle]
pub extern "C" fn rust_murmur3_128(input: *const c_void, len: usize, seed: u64, out: *mut c_void) {
    let bytes = byte_slice!(input, len);
    let hash_bytes = murmurhash3_128(bytes, seed).to_ne_bytes();
    copy_to_output!(out, hash_bytes);
}

#[no_mangle]
pub extern "C" fn rust_murmur3_32(input: *const c_void, len: usize, seed: u32, out: *mut c_void) {
    let bytes = byte_slice!(input, len);
    let hash_bytes = murmurhash3_32(bytes, seed).to_ne_bytes();
    copy_to_output!(out, hash_bytes);
}

#[no_mangle]
pub extern "C" fn rust_crc32c(input: *const c_void, len: usize, seed: u32, out: *mut c_void) {
    let bytes = byte_slice!(input, len);
    let hash_bytes = crc32c_with_initial(bytes, seed).to_ne_bytes();
    copy_to_output!(out, hash_bytes);
}

#[no_mangle]
pub extern "C" fn rust_cityhash_64(input: *const c_void, len: usize, seed: u64, out: *mut c_void) {
    let bytes = byte_slice!(input, len);
    let hash_bytes = cityhash_64_with_seed(bytes, 0x9ae16a3b2f90404f_u64, seed).to_ne_bytes();
    copy_to_output!(out, hash_bytes);
}

#[no_mangle]
pub extern "C" fn rust_cityhash_64_no_seed(input: *const c_void, len: usize, out: *mut c_void) {
    let bytes = byte_slice!(input, len);
    let hash_bytes = cityhash_64(bytes).to_ne_bytes();
    copy_to_output!(out, hash_bytes);
}
