//! NIST SP800-22 Ã‚Â§2.7 Ã¢â‚¬â€ Non-overlapping Template Matching Test (stub).
//!
//! Counts the number of occurrences of a pre-specified target bit string (template)
//! in the sequence. The sequence is partitioned into non-overlapping blocks, and
//! each block is scanned for the template independently.
//!
//! **Status: Stub** Ã¢â‚¬â€ accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Non-overlapping template matching accumulator (stub).
///
/// Template length `m` is typically 9 or 10 bits for NIST compliance.
#[derive(Debug, Clone, Copy, Default)]
pub struct NonOverlappingTemplateTest {
    pub total_bits: u64,
}

impl NonOverlappingTemplateTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for NonOverlappingTemplateTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bits += (chunk.len() as u64) * 8;
    }

    fn reset(&mut self) {
        self.total_bits = 0;
    }

    fn evaluate(&self) -> TestResult {
        // TODO: scan each M-bit block for the template, compute chi-square vs
        //       expected occurrences using the NIST SP800-22 Ã‚Â§2.7.4 formula.
        TestResult::NOT_IMPLEMENTED
    }
}
