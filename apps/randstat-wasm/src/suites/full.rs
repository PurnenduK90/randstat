//! WASM C-ABI exports for Full combined Suite.

use randstat_suite_full::FullSuite;

static mut FULL_SUITE: FullSuite = FullSuite::ZERO;

#[no_mangle]
pub extern "C" fn full_reset() {
    unsafe {
        FULL_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn full_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        FULL_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// # Safety: `out_eval` must point to a writable `FullEvaluation` region.
#[no_mangle]
pub unsafe extern "C" fn full_finalize(out_eval: *mut randstat_suite_full::FullEvaluation) {
    if out_eval.is_null() {
        return;
    }
    unsafe {
        *out_eval = FULL_SUITE.evaluate();
    }
}
