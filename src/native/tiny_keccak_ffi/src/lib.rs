// src/lib.rs

use tiny_keccak::{Hasher, Keccak};

#[no_mangle]
pub extern "C" fn keccak_256(input_ptr: *const u8, input_len: usize, output_ptr: *mut u8) {
    let input = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let mut keccak = Keccak::v256();

    keccak.update(input);

    let mut output = [0u8; 32];
    keccak.finalize(&mut output);

    unsafe {
        std::ptr::copy_nonoverlapping(output.as_ptr(), output_ptr, 32);
    }
}

