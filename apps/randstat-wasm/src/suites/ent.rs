//! WASM C-ABI exports for Fourmilab ENT Suite.

use randstat_core::bitstream::sha256::Sha256;
use randstat_suite_ent::EntSuite;

static mut ENT_SUITE: EntSuite = EntSuite::ZERO;
static mut SHA256_HASHER: Sha256 = Sha256::new();

#[no_mangle]
pub extern "C" fn ent_reset() {
    unsafe {
        ENT_SUITE.reset();
        SHA256_HASHER.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn ent_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        let slice = core::slice::from_raw_parts(buffer_ptr, len);
        ENT_SUITE.update(slice);
        SHA256_HASHER.update(slice);
    }
}

/// # Safety: `out_result` must point to a writable `EntResult`-sized region.
#[no_mangle]
pub unsafe extern "C" fn ent_finalize(
    out_result: *mut randstat_core::stats::ent_result::EntResult,
) {
    if out_result.is_null() {
        return;
    }
    unsafe {
        if let Some(mut r) = ENT_SUITE.finalize() {
            r.sha256 = SHA256_HASHER.finalize();
            *out_result = r;
        }
    }
}

/// # Safety: `out_guardrail` must point to a writable `GuardrailEvaluation`-sized region.
#[no_mangle]
pub unsafe extern "C" fn ent_validate(
    alpha: f64,
    out_guardrail: *mut randstat_core::stats::validate::GuardrailEvaluation,
) {
    if out_guardrail.is_null() {
        return;
    }
    unsafe {
        if let Some(e) = ENT_SUITE.evaluate(alpha) {
            *out_guardrail = e;
        }
    }
}
