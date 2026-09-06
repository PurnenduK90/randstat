//! WASM C-ABI exports for Quick Randomness Screening & Health Diagnostic Suite.

use randstat_suite_quick::QuickSuite;

static mut QUICK_SUITE: QuickSuite = QuickSuite::ZERO;

#[no_mangle]
pub extern "C" fn quick_reset() {
    unsafe {
        QUICK_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn quick_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        QUICK_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// # Safety: `out_eval` must point to a writable `QuickEvaluation` region in WASM memory.
#[no_mangle]
pub unsafe extern "C" fn quick_finalize(out_eval: *mut randstat_suite_quick::QuickEvaluation) {
    if out_eval.is_null() {
        return;
    }
    unsafe {
        *out_eval = QUICK_SUITE.evaluate();
    }
}

/// # Safety: `out_eval` must point to a writable `GuardrailEvaluation` region in WASM memory.
#[no_mangle]
pub unsafe extern "C" fn quick_validate(
    alpha: f64,
    out_eval: *mut randstat_core::stats::validate::GuardrailEvaluation,
) {
    if out_eval.is_null() {
        return;
    }
    unsafe {
        if let Some(e) = QUICK_SUITE.validate(alpha) {
            *out_eval = e;
        }
    }
}
