//! WASM C-ABI exports for DIEHARD / Dieharder Suite.

use randstat_suite_dieharder::DieharderSuite;

static mut DIEHARDER_SUITE: DieharderSuite = DieharderSuite::ZERO;

#[no_mangle]
pub extern "C" fn dieharder_reset() {
    unsafe {
        DIEHARDER_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn dieharder_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        DIEHARDER_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// # Safety: `out_eval` must point to a writable `DieharderEvaluation` region.
#[no_mangle]
pub unsafe extern "C" fn dieharder_finalize(
    out_eval: *mut randstat_suite_dieharder::DieharderEvaluation,
) {
    if out_eval.is_null() {
        return;
    }
    unsafe {
        *out_eval = DIEHARDER_SUITE.evaluate();
    }
}
