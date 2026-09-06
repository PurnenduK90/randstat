//! WASM C-ABI exports for NIST SP 800-22 Rev 1a Suite.

use randstat_suite_nist::NistSuite;

static mut NIST_SUITE: NistSuite = NistSuite::ZERO;

#[no_mangle]
pub extern "C" fn nist_reset() {
    unsafe {
        NIST_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn nist_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        NIST_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// Finalises the NIST SP800-22 evaluation and writes the structured result to `out_eval`.
///
/// # Safety
/// `out_eval` must point to a writable `NistEvaluation` region in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn nist_finalize(out_eval: *mut randstat_suite_nist::NistEvaluation) {
    if out_eval.is_null() {
        return;
    }
    unsafe {
        *out_eval = NIST_SUITE.evaluate();
    }
}
