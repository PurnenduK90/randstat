//! WASM C-ABI exports for NIST SP800-90B Suite.

use randstat_suite_sp800_90b::Sp80090bSuite;

static mut SP80090B_SUITE: Sp80090bSuite = Sp80090bSuite::ZERO;

#[no_mangle]
pub extern "C" fn sp80090b_reset() {
    unsafe {
        SP80090B_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn sp80090b_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        SP80090B_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// # Safety: `out_eval` must point to a writable `Sp80090bEvaluation` region.
#[no_mangle]
pub unsafe extern "C" fn sp80090b_finalize(
    out_eval: *mut randstat_suite_sp800_90b::Sp80090bEvaluation,
) {
    if out_eval.is_null() {
        return;
    }
    unsafe {
        *out_eval = SP80090B_SUITE.evaluate();
    }
}
