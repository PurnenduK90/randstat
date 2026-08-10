//! Dieharder: OPERM5 — Overlapping Permutations of 5 Test (stub).
//!
//! Takes overlapping groups of 5 consecutive integers from the stream and
//! records which of the 120 possible orderings (permutations of 5 elements)
//! they form. The frequency of each permutation should be approximately equal
//! for a truly random source (chi-square test against uniform expected count).
//!
//! Highly sensitive to weak linear congruential generators and short-period PRNGs.
//!
//! **Status: Stub** — accumulates total bytes; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// OPERM5 Test accumulator (stub).
///
/// Each sample uses 5 consecutive 4-byte integers; window slides by 1 integer.
#[derive(Debug, Clone, Copy, Default)]
pub struct Operm5Test {
    pub total_bytes: u64,
}

impl Operm5Test {
    pub const fn new() -> Self {
        Self { total_bytes: 0 }
    }
}

impl StreamTest for Operm5Test {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
    }
    fn reset(&mut self) {
        self.total_bytes = 0;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: read stream as u32 values; for each overlapping window of 5,
        //       determine the rank ordering (1 of 120 permutations);
        //       accumulate 120-bin frequency table; chi-square vs expected.
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
