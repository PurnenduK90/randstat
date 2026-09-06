//! WASM C-ABI exports for gjrand Suite.

use randstat_suite_gjrand::GjrandSuite;

static mut GJRAND_SUITE: GjrandSuite = GjrandSuite::ZERO;

#[no_mangle]
pub extern "C" fn gjrand_reset() {
    unsafe {
        GJRAND_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn gjrand_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        GJRAND_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// # Safety: `out_eval` must point to a writable `GjrandEvaluation` region.
#[no_mangle]
pub unsafe extern "C" fn gjrand_finalize(out_eval: *mut randstat_suite_gjrand::GjrandEvaluation) {
    if out_eval.is_null() {
        return;
    }
    unsafe {
        *out_eval = GJRAND_SUITE.evaluate();
    }
}
