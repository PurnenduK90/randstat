//! WASM C-ABI exports for TestU01 Suite.

use randstat_suite_testu01::TestU01Suite;

static mut TESTU01_SUITE: TestU01Suite = TestU01Suite::ZERO;

#[no_mangle]
pub extern "C" fn testu01_reset() {
    unsafe {
        TESTU01_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn testu01_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        TESTU01_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// # Safety: `out_eval` must point to a writable `TestU01Evaluation` region.
#[no_mangle]
pub unsafe extern "C" fn testu01_finalize(
    out_eval: *mut randstat_suite_testu01::TestU01Evaluation,
) {
    if out_eval.is_null() {
        return;
    }
    unsafe {
        *out_eval = TESTU01_SUITE.evaluate();
    }
}
