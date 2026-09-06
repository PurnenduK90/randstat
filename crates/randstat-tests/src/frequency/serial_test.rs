//! NIST SP800-22 Ã‚Â§2.11 Ã¢â‚¬â€ Serial Test (stub).
//!
//! Tests whether the frequency of all possible overlapping `m`-bit patterns
//! across the bit string is approximately equal. This is the bit-level
//! generalisation of the frequency test.
//!
//! Note: this is **not** the same as the Serial Correlation test (which is
//! a byte-level Pearson correlation). This is a frequency test on m-bit tuples.
//!
//! **Status: Stub** Ã¢â‚¬â€ accumulates total bits; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Serial Test (m-bit pattern frequency) accumulator (stub).
///
/// Typical parameter: `m = 16` (tests all 65536 possible 16-bit patterns).
#[derive(Debug, Clone, Copy, Default)]
pub struct SerialTest {
    pub total_bits: u64,
}

impl SerialTest {
    pub const fn new() -> Self {
        Self { total_bits: 0 }
    }
}

impl StreamTest for SerialTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bits += (chunk.len() as u64) * 8;
    }

    fn reset(&mut self) {
        self.total_bits = 0;
    }

    fn evaluate(&self) -> TestResult {
        // TODO: build overlapping m-bit and (m-1)-bit pattern frequency tables.
        //       Compute del_m and del_{m-1} psiÃ‚Â² statistics (Ã‚Â§2.11.4 formula).
        //       Two p-values produced; test passes if both Ã¢â€°Â¥ alpha.
        TestResult::NOT_IMPLEMENTED
    }
}
