//! WASM C-ABI exports for SHA-256 computation.

use randstat_core::bitstream::sha256::Sha256;

static mut SHA256_HASHER: Sha256 = Sha256::new();

/// Reset the SHA-256 hasher to its initial state.
#[no_mangle]
pub extern "C" fn sha256_reset() {
    unsafe {
        SHA256_HASHER.reset();
    }
}

/// Feed `len` bytes at `buffer_ptr` into the SHA-256 hasher.
///
/// # Safety
/// `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn sha256_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        SHA256_HASHER.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// Finalise the digest and write 32 bytes to `out_ptr`.
///
/// The hasher is **not** reset after this call — call `sha256_reset()` to start a new digest.
///
/// # Safety
/// `out_ptr` must point to at least 32 writable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn sha256_finalize(out_ptr: *mut u8) {
    if out_ptr.is_null() {
        return;
    }
    unsafe {
        let digest = SHA256_HASHER.finalize();
        core::ptr::copy_nonoverlapping(digest.as_ptr(), out_ptr, 32);
    }
}
