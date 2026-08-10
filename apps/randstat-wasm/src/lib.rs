//! `randstat-wasm` — `#![no_std]` WASM C-ABI bindings.
//!
//! Exposes mathematical and statistical functions to JavaScript via the WebAssembly
//! C ABI. All state lives in WASM linear memory (static globals) — no heap allocation.
//!
//! # Feature Flags
//!
//! | Feature | Exports included | Suite crate pulled in |
//! |---|---|---|
//! | `math-only` (default) | Chi2/Normal PDF, CDF, critical values, plot generators | none |
//! | `ent` | math-only + ENT streaming engine | `randstat-suite-ent` |
//! | `nist` | math-only + NIST streaming engine | `randstat-suite-nist` |
//! | `quick` | math-only + 4-test fast screening | `randstat-suite-quick` |
//! | `full` | math-only + ENT + all NIST tests | `randstat-suite-full` |
//! | `dieharder` | math-only + Dieharder stubs | `randstat-suite-dieharder` |
//!
//! # Building
//! ```bash
//! # Use the workspace justfile:
//! just build-wasm-all
//!
//! # Or individually:
//! cargo build -p randstat-wasm --no-default-features --features math-only \
//!     --target wasm32-unknown-unknown --release
//! cargo build -p randstat-wasm --no-default-features --features ent \
//!     --target wasm32-unknown-unknown --release
//! ```

#![no_std]
#![allow(static_mut_refs)]

// ─── Panic handler (wasm32 only; not during `cargo test`) ────────────────────

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// ─── Shared plot-point buffer (500 x,y pairs) ────────────────────────────────

static mut POINT_BUFFER: [f64; 1000] = [0.0f64; 1000];

// ─── SHA-256 hasher — always present, not feature-gated ──────────────────────
//
// SHA-256 is file identity, not a statistical test. It is compiled into every
// WASM build (including math-only) so callers can always compute a digest
// independently of whichever suite is active.

use randstat_core::bitstream::sha256::Sha256;
static mut SHA256_HASHER: Sha256 = Sha256::new();

/// Reset the SHA-256 hasher to its initial state.
#[no_mangle]
pub extern "C" fn sha256_reset() {
    unsafe {
        SHA256_HASHER.reset();
    }
}

/// Feed `len` bytes at `buffer_ptr` into the SHA-256 hasher.
///
/// # Safety
/// `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn sha256_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        SHA256_HASHER.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

/// Finalise the digest and write 32 bytes to `out_ptr`.
///
/// The hasher is **not** reset after this call — call `sha256_reset()` to start a new digest.
///
/// # Safety
/// `out_ptr` must point to at least 32 writable bytes in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn sha256_finalize(out_ptr: *mut u8) {
    if out_ptr.is_null() {
        return;
    }
    unsafe {
        let digest = SHA256_HASHER.finalize();
        core::ptr::copy_nonoverlapping(digest.as_ptr(), out_ptr, 32);
    }
}

#[cfg(feature = "ent")]
use randstat_suite_ent::EntSuite;
#[cfg(feature = "ent")]
static mut ENT_SUITE: EntSuite = EntSuite::ZERO;

#[cfg(feature = "nist")]
use randstat_suite_nist::NistSuite;
#[cfg(feature = "nist")]
static mut NIST_SUITE: NistSuite = NistSuite::ZERO;

#[cfg(feature = "quick")]
use randstat_suite_quick::QuickSuite;
#[cfg(feature = "quick")]
static mut QUICK_SUITE: QuickSuite = QuickSuite::ZERO;

#[cfg(feature = "full")]
use randstat_suite_full::FullSuite;
#[cfg(feature = "full")]
static mut FULL_SUITE: FullSuite = FullSuite::ZERO;

// ─── Math-only exports (compiled for all features) ───────────────────────────

use randstat_core::math::chi2::{chi2_critical_value, chi2_pdf, normal_pdf, pochisq};
use randstat_core::math::plot::{generate_chi2_points, generate_normal_points};

/// Chi-Square PDF at `x` with `df` degrees of freedom.
#[no_mangle]
pub extern "C" fn chi2_pdf_export(x: f64, df: f64) -> f64 {
    chi2_pdf(x, df)
}

/// Normal distribution PDF at `x` with mean=`df`, σ=√(2·df).
#[no_mangle]
pub extern "C" fn normal_pdf_export(x: f64, df: f64) -> f64 {
    normal_pdf(x, df)
}

/// Chi-square critical value (Wilson-Hilferty) at `alpha` significance, `df` degrees.
#[no_mangle]
pub extern "C" fn chi2_critical_value_export(alpha: f64, df: f64) -> f64 {
    chi2_critical_value(alpha, df)
}

/// Upper-tail chi-square probability P(χ² > x | df).
#[no_mangle]
pub extern "C" fn chi2_pochisq(x: f64, df: usize) -> f64 {
    pochisq(x, df)
}

/// Pointer to the shared 1000-element f64 plot buffer. Read after calling a `generate_*` fn.
#[no_mangle]
pub extern "C" fn get_buffer_ptr() -> *const f64 {
    unsafe { POINT_BUFFER.as_ptr() }
}

/// Fills buffer with `steps` interleaved (x,y) pairs for Chi-Square PDF. Returns values written.
#[no_mangle]
pub extern "C" fn wasm_generate_chi2_points(
    df: f64,
    x_min: f64,
    x_max: f64,
    steps: usize,
) -> usize {
    unsafe { generate_chi2_points(df, x_min, x_max, steps, &mut POINT_BUFFER) }
}

/// Fills buffer with `steps` interleaved (x,y) pairs for Normal approx PDF. Returns values written.
#[no_mangle]
pub extern "C" fn wasm_generate_normal_points(
    df: f64,
    x_min: f64,
    x_max: f64,
    steps: usize,
) -> usize {
    unsafe { generate_normal_points(df, x_min, x_max, steps, &mut POINT_BUFFER) }
}

// ─── ENT suite exports ────────────────────────────────────────────────────────

#[cfg(feature = "ent")]
#[no_mangle]
pub extern "C" fn ent_reset() {
    unsafe {
        ENT_SUITE.reset();
        SHA256_HASHER.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[cfg(feature = "ent")]
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
#[cfg(feature = "ent")]
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
#[cfg(feature = "ent")]
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

// ─── NIST suite exports ───────────────────────────────────────────────────────

#[cfg(feature = "nist")]
#[no_mangle]
pub extern "C" fn nist_reset() {
    unsafe {
        NIST_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[cfg(feature = "nist")]
#[no_mangle]
pub unsafe extern "C" fn nist_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        NIST_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

// ─── Quick suite exports ──────────────────────────────────────────────────────

#[cfg(feature = "quick")]
#[no_mangle]
pub extern "C" fn quick_reset() {
    unsafe {
        QUICK_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[cfg(feature = "quick")]
#[no_mangle]
pub unsafe extern "C" fn quick_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        QUICK_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

// ─── Full suite exports ───────────────────────────────────────────────────────

#[cfg(feature = "full")]
#[no_mangle]
pub extern "C" fn full_reset() {
    unsafe {
        FULL_SUITE.reset();
    }
}

/// # Safety: `buffer_ptr` must point to `len` readable bytes in WASM linear memory.
#[cfg(feature = "full")]
#[no_mangle]
pub unsafe extern "C" fn full_update(buffer_ptr: *const u8, len: usize) {
    if buffer_ptr.is_null() || len == 0 {
        return;
    }
    unsafe {
        FULL_SUITE.update(core::slice::from_raw_parts(buffer_ptr, len));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "ent")]
    use randstat_core::stats::ent_result::EntResult;
    #[cfg(feature = "ent")]
    use randstat_core::stats::validate::GuardrailEvaluation;

    #[test]
    fn test_wasm_exports() {
        // SHA-256
        sha256_reset();
        let data = [1, 2, 3, 4, 5];
        unsafe {
            sha256_update(data.as_ptr(), data.len());
            // Null/zero checks
            sha256_update(core::ptr::null(), 0);
            sha256_update(data.as_ptr(), 0);
            sha256_update(core::ptr::null(), 10);
            
            let mut out = [0u8; 32];
            sha256_finalize(out.as_mut_ptr());
            sha256_finalize(core::ptr::null_mut());
            assert_ne!(out, [0u8; 32]);
        }

        // Math
        assert!(chi2_pdf_export(2.0, 2.0) > 0.0);
        assert!(normal_pdf_export(0.0, 2.0) > 0.0);
        assert!(chi2_critical_value_export(0.05, 255.0) > 0.0);
        assert!(chi2_pochisq(2.0, 2) > 0.0);
        assert_eq!(get_buffer_ptr(), unsafe { POINT_BUFFER.as_ptr() });

        assert!(wasm_generate_chi2_points(2.0, 0.0, 1.0, 5) > 0);
        assert!(wasm_generate_normal_points(2.0, 0.0, 1.0, 5) > 0);

        // Feature ENT
        #[cfg(feature = "ent")]
        unsafe {
            ent_reset();
            ent_update(data.as_ptr(), data.len());
            ent_update(core::ptr::null(), 0);
            ent_update(data.as_ptr(), 0);
            ent_update(core::ptr::null(), 10);

            let mut res = core::mem::zeroed::<EntResult>();
            ent_finalize(&mut res);
            ent_finalize(core::ptr::null_mut());
            assert_eq!(res.total_bytes, 5);

            let mut eval = core::mem::zeroed::<GuardrailEvaluation>();
            ent_validate(0.05, &mut eval);
            ent_validate(0.05, core::ptr::null_mut());
            assert_eq!(eval.pi_error_percent, 100.0);
        }

        // Feature NIST
        #[cfg(feature = "nist")]
        unsafe {
            nist_reset();
            nist_update(data.as_ptr(), data.len());
            nist_update(core::ptr::null(), 0);
        }

        // Feature QUICK
        #[cfg(feature = "quick")]
        unsafe {
            quick_reset();
            quick_update(data.as_ptr(), data.len());
            quick_update(core::ptr::null(), 0);
        }

        // Feature FULL
        #[cfg(feature = "full")]
        unsafe {
            full_reset();
            full_update(data.as_ptr(), data.len());
            full_update(core::ptr::null(), 0);
        }
    }
}
