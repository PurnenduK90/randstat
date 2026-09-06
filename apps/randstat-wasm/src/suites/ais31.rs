//! WASM C-ABI exports for BSI AIS 20 / AIS 31 TRNG Suite.

use randstat_suite_ais31::Ais31Suite;

static mut AIS31_SUITE: Ais31Suite = Ais31Suite::ZERO;

#[no_mangle]
pub extern "C" fn ais31_reset() {
    unsafe {
        AIS31_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn ais31_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        AIS31_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// Finalises the BSI AIS 20 / AIS 31 evaluation and writes the structured result to `out_eval`.
///
/// # Safety
/// `out_eval` must point to a writable `Ais31Evaluation` region in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn ais31_finalize(out_eval: *mut randstat_suite_ais31::Ais31Evaluation) {
    if out_eval.is_null() {
        return;
    }
    unsafe {
        *out_eval = AIS31_SUITE.evaluate();
    }
}
