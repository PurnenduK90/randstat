//! NIST SP800-22 §2.8 — Overlapping Template Matching Test (stub).
//!
//! Similar to the non-overlapping template test, but the window slides by one
//! bit at a time (overlapping). This is more sensitive to periodic patterns.
//! Uses the template of `m` ones as the target template.
//!
//! **Status: Stub** — accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Overlapping template matching accumulator (stub).
///
/// Default template: `m` consecutive 1-bits (e.g. `1111111111` for m=10).
#[derive(Debug, Clone, Copy, Default)]
pub struct OverlappingTemplateTest {
    pub total_bits: u64,
}

impl OverlappingTemplateTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for OverlappingTemplateTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bits += (chunk.len() as u64) * 8;
    }

    fn reset(&mut self) {
        self.total_bits = 0;
    }

    fn evaluate(&self) -> TestResult {
        // TODO: slide a 1-bit window counting overlapping template matches.
        //       Compute p-value using the Psi distribution from §2.8.4.
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
