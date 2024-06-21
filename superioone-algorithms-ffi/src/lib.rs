use algorithms::hash::{
    cityhash::hash_fn::{
        cityhash_128, cityhash_128_with_seed, cityhash_32, cityhash_64, cityhash_64_with_seed,
        cityhash_crc128_with_seed, cityhash_crc256,
    },
    crc::hash_fn::crc32c_with_initial,
    murmur3::hash_fn::{murmurhash3_128, murmurhash3_32},
};
use std::ffi::c_void;
use std::mem::size_of;
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
pub extern "C" fn rust_cityhash_32(input: *const c_void, len: usize, seed: u32, out: *mut c_void) {
    let bytes = byte_slice!(input, len);
    let hash_bytes = cityhash_32(bytes, seed).to_ne_bytes();
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

#[no_mangle]
pub extern "C" fn rust_cityhash_128_no_seed(input: *const c_void, len: usize, out: *mut c_void) {
    let bytes = byte_slice!(input, len);
    let hash_bytes = cityhash_128(bytes).to_ne_bytes();
    copy_to_output!(out, hash_bytes);
}

#[no_mangle]
pub extern "C" fn rust_cityhash_128(
    input: *const c_void,
    len: usize,
    seed0: u64,
    seed1: u64,
    out: *mut c_void,
) {
    let bytes = byte_slice!(input, len);
    let hash_bytes = cityhash_128_with_seed(bytes, seed0, seed1).to_ne_bytes();
    copy_to_output!(out, hash_bytes);
}

#[no_mangle]
pub extern "C" fn rust_cityhash_128crc(
    input: *const c_void,
    len: usize,
    seed0: u64,
    seed1: u64,
    out: *mut c_void,
) {
    let bytes = byte_slice!(input, len);
    let hash_bytes = cityhash_crc128_with_seed(bytes, seed0, seed1).to_ne_bytes();
    copy_to_output!(out, hash_bytes);
}

#[no_mangle]
pub extern "C" fn rust_cityhash_256crc(input: *const c_void, len: usize, out: *mut c_void) {
    let bytes = byte_slice!(input, len);
    let hash = cityhash_crc256(bytes);
    let hash_bytes: &[u64; 4] = &[hash.0, hash.1, hash.2, hash.3];
    unsafe {
        copy_nonoverlapping(
            hash_bytes.as_ptr().cast::<u8>(),
            out as *mut u8,
            hash_bytes.len() * size_of::<u64>(),
        )
    };
}
