//! `randstat-wasm` — `#![no_std]` WASM C-ABI bindings.
//!
//! Exposes mathematical and statistical functions to JavaScript via the WebAssembly
//! C ABI. All state lives in WASM linear memory (static globals) — zero heap allocation.
//!
//! # Architecture
//! - [`math`] — Chi2/Normal PDF, CDF, critical values, plot generation
//! - [`sha256`] — Streaming SHA-256 file hashing
//! - [`suites`] — Modular, feature-gated randomness suites (ENT, NIST, AIS 31, etc.)

#![no_std]
#![allow(static_mut_refs)]

// ─── Panic handler (wasm32 only; not during `cargo test`) ────────────────────

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub mod generators;
pub mod math;
pub mod sha256;
pub mod suites;

static mut INPUT_BUFFER: [u8; 65536] = [0u8; 65536];
static mut EVAL_BUFFER: [u8; 8192] = [0u8; 8192];

/// Pointer to a safe 64 KB static input buffer in WASM memory.
#[no_mangle]
pub extern "C" fn get_input_buffer_ptr() -> *mut u8 {
    unsafe { INPUT_BUFFER.as_mut_ptr() }
}

/// Capacity of the static input buffer (65536 bytes).
#[no_mangle]
pub extern "C" fn get_input_buffer_capacity() -> usize {
    65536
}

/// Pointer to a safe 8 KB static evaluation buffer in WASM memory.
#[no_mangle]
pub extern "C" fn get_eval_buffer_ptr() -> *mut u8 {
    unsafe { EVAL_BUFFER.as_mut_ptr() }
}

// Re-export C-ABI symbols
pub use generators::*;
pub use math::*;
pub use sha256::*;

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
        assert!(!get_buffer_ptr().is_null());

        assert!(wasm_generate_chi2_points(2.0, 0.0, 1.0, 5) > 0);
        assert!(wasm_generate_normal_points(2.0, 0.0, 1.0, 5) > 0);

        // Feature ENT
        #[cfg(feature = "ent")]
        unsafe {
            ent::ent_reset();
            ent::ent_update(data.as_ptr(), data.len());
            ent::ent_update(core::ptr::null(), 0);
            ent::ent_update(data.as_ptr(), 0);
            ent::ent_update(core::ptr::null(), 10);

            let mut res = core::mem::zeroed::<EntResult>();
            ent::ent_finalize(&mut res);
            ent::ent_finalize(core::ptr::null_mut());
            assert_eq!(res.total_bytes, 5);

            let mut eval = core::mem::zeroed::<GuardrailEvaluation>();
            ent::ent_validate(0.05, &mut eval);
            ent::ent_validate(0.05, core::ptr::null_mut());
            assert_eq!(eval.pi_error_percent, 100.0);
        }

        // Feature NIST
        #[cfg(feature = "nist")]
        unsafe {
            nist::nist_reset();
            nist::nist_update(data.as_ptr(), data.len());
            nist::nist_update(core::ptr::null(), 0);
            let mut eval = core::mem::zeroed::<randstat_suite_nist::NistEvaluation>();
            nist::nist_finalize(&mut eval);
            nist::nist_finalize(core::ptr::null_mut());
            assert_eq!(eval.total_tests, 15);
        }

        // Feature AIS31
        #[cfg(feature = "ais31")]
        unsafe {
            ais31::ais31_reset();
            ais31::ais31_update(data.as_ptr(), data.len());
            ais31::ais31_update(core::ptr::null(), 0);
            let mut eval = core::mem::zeroed::<randstat_suite_ais31::Ais31Evaluation>();
            ais31::ais31_finalize(&mut eval);
            ais31::ais31_finalize(core::ptr::null_mut());
            assert_eq!(eval.total_tests, 9);
        }

        // Feature FULL
        #[cfg(feature = "full")]
        unsafe {
            full::full_reset();
            full::full_update(data.as_ptr(), data.len());
            full::full_update(core::ptr::null(), 0);
            let mut eval = core::mem::zeroed::<randstat_suite_full::FullEvaluation>();
            full::full_finalize(&mut eval);
            full::full_finalize(core::ptr::null_mut());
        }

        // Generator WASM exports
        unsafe {
            // Null safety
            assert_eq!(generator_fill(core::ptr::null_mut(), 10), 0);
            assert_eq!(generator_fill(core::ptr::null_mut(), 0), 0);

            // Test each generator type init and fill
            for gen_type in 0..=10 {
                assert_eq!(generator_init(gen_type, 1, 1, 12345, 67890), 1);
                let count = generator_fill_chunk(100);
                assert_eq!(count, 100);
                assert!(generator_format_chunk(10, 1) >= 20); // decimal (20-40 bytes)
                assert_eq!(generator_format_chunk(10, 2), 10 * 8); // bits
                assert_eq!(generator_format_chunk(10, 3), 10 * 2); // hex
                generator_reset();
            }

            assert!(!generator_get_chunk_ptr().is_null());
            assert!(!generator_get_formatted_ptr().is_null());
            assert!(generator_get_chunk_capacity() > 0);
            assert!(generator_get_formatted_capacity() > 0);
        }
    }
}
