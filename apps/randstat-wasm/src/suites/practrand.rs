//! WASM C-ABI exports for PractRand Suite.

use randstat_suite_practrand::PractRandSuite;

static mut PRACTRAND_SUITE: PractRandSuite = PractRandSuite::ZERO;

#[no_mangle]
pub extern "C" fn practrand_reset() {
    unsafe {
        PRACTRAND_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn practrand_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        PRACTRAND_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// # Safety: `out_eval` must point to a writable `PractRandEvaluation` region.
#[no_mangle]
pub unsafe extern "C" fn practrand_finalize(
    out_eval: *mut randstat_suite_practrand::PractRandEvaluation,
) {
    if out_eval.is_null() {
        return;
    }
    unsafe {
        *out_eval = PRACTRAND_SUITE.evaluate();
    }
}
