//! `randstat-suite-ent` — Zero-alloc ENT test suite.
//!
//! Aggregates the three tests from the original Fourmilab ENT program:
//! - Shannon entropy (bits per byte)
//! - Monte Carlo π estimation
//! - Serial correlation coefficient
//!
//! **SHA-256 is deliberately NOT included here.** It is a file identity
//! mechanism, not a statistical test. Callers (CLI, WASM) maintain their own
//! independent [`randstat_core::bitstream::sha256::Sha256`] accumulator and
//! set [`EntResult::sha256`] themselves after calling [`EntSuite::finalize`].
//!
//! Used by the default WASM ENT build and the CLI. Pulling this crate in does
//! NOT bring in any NIST stubs — the linker sees only what this crate imports.

#![no_std]

use randstat_core::math::chi2::compute_chi_square;
use randstat_core::stats::ent_result::EntResult;
use randstat_core::stats::validate::{evaluate_guardrails, GuardrailEvaluation};
use randstat_core::traits::StreamTest;
use randstat_tests::frequency::shannon_entropy::ShannonEntropyTest;
use randstat_tests::spatial::monte_carlo_pi::MonteCarloPiTest;
use randstat_tests::spatial::serial_correlation::SerialCorrelationTest;

/// Zero-alloc ENT test suite (Shannon entropy + Monte Carlo π + Serial correlation).
///
/// SHA-256 is intentionally absent — it is computed independently by the
/// caller and set on the returned [`EntResult`] after [`finalize`](EntSuite::finalize).
///
/// All state lives on the stack / in a WASM `static mut` — no heap allocation.
/// Use [`EntSuite::ZERO`] as a `static mut` initialiser in WASM.
#[derive(Debug, Clone, Copy)]
pub struct EntSuite {
    pub shannon: ShannonEntropyTest,
    pub monte_carlo: MonteCarloPiTest,
    pub serial_correlation: SerialCorrelationTest,
    /// Running byte sum — used to compute arithmetic mean for the report.
    pub sum_x: f64,
}

impl EntSuite {
    /// Const zero-initialised instance — safe for `static mut` in WASM.
    pub const ZERO: Self = Self {
        shannon: ShannonEntropyTest::new(),
        monte_carlo: MonteCarloPiTest::new(),
        serial_correlation: SerialCorrelationTest::new(),
        sum_x: 0.0,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    /// Feeds a chunk of bytes into all statistical accumulators.
    ///
    /// Note: SHA-256 is NOT updated here — call your own
    /// `Sha256::update(chunk)` in parallel if you need file identity.
    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.shannon.update(chunk);
        self.monte_carlo.update(chunk);
        self.serial_correlation.update(chunk);
        for &b in chunk {
            self.sum_x += b as f64;
        }
    }

    /// Resets all statistical accumulators to their zero state.
    pub fn reset(&mut self) {
        self.shannon.reset();
        self.monte_carlo.reset();
        self.serial_correlation.reset();
        self.sum_x = 0.0;
    }

    /// Produces an [`EntResult`] from the current accumulator state.
    ///
    /// Returns `None` if no bytes have been processed.
    ///
    /// **`result.sha256` is zero-initialised.** The caller is responsible for
    /// computing SHA-256 independently and setting this field:
    /// ```ignore
    /// let mut result = suite.finalize().unwrap();
    /// result.sha256 = my_sha256_accumulator.finalize();
    /// ```
    pub fn finalize(&self) -> Option<EntResult> {
        let tracker = &self.shannon.tracker;
        if tracker.total_bytes == 0 {
            return None;
        }

        let total = tracker.total_bytes as f64;
        let entropy_val = tracker.entropy();
        Some(EntResult {
            total_bytes: tracker.total_bytes,
            entropy_bits_per_byte: entropy_val,
            compression_percent: (8.0 - entropy_val) / 8.0 * 100.0,
            chi_square: compute_chi_square(&tracker.byte_counts, tracker.total_bytes),
            mean: self.sum_x / total,
            monte_carlo_pi: self.monte_carlo.accum.pi(),
            serial_correlation: self.serial_correlation.accum.correlation(),
            sha256: [0u8; 32], // caller fills this in
        })
    }

    /// Evaluates guardrails at significance level `alpha`.
    /// Returns `None` if no bytes have been processed.
    pub fn evaluate(&self, alpha: f64) -> Option<GuardrailEvaluation> {
        self.finalize().map(|res| evaluate_guardrails(&res, alpha))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ent_suite() {
        let suite = EntSuite::new();
        assert!(suite.finalize().is_none());
        assert!(suite.evaluate(0.05).is_none());

        // Default impl
        let mut default_suite = EntSuite::default();
        default_suite.update(&[1, 2, 3, 4, 5]);
        let res = default_suite.finalize().unwrap();
        assert_eq!(res.total_bytes, 5);

        let eval = default_suite.evaluate(0.05).unwrap();
        assert_eq!(eval.pi_error_percent, 100.0);

        default_suite.reset();
        assert!(default_suite.finalize().is_none());
    }
}

impl Default for EntSuite {
    fn default() -> Self {
        Self::new()
    }
}
