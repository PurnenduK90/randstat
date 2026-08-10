//! Dieharder: DNA Test (stub).
//!
//! Marsaglia's DNA test. Generates 10-letter "words" from a 4-letter alphabet
//! {A, C, G, T} by mapping each 2-bit pair of a byte to a letter (00→A, 01→C,
//! 10→G, 11→T). For each 10-letter word, counts occurrences.
//!
//! The number of missing words (words that never appear) should follow a
//! known distribution for a truly random source.
//!
//! **Status: Stub** — accumulates total bytes; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// DNA Test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct DnaTest {
    pub total_bytes: u64,
}

impl DnaTest {
    pub const fn new() -> Self {
        Self { total_bytes: 0 }
    }
}

impl StreamTest for DnaTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
    }
    fn reset(&mut self) {
        self.total_bytes = 0;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: decode bytes as 4 letters (2 bits each); form overlapping 10-letter words;
        //       maintain a 4^10 = 1_048_576 element bitset of seen words;
        //       count missing words and compare to expected Poisson distribution.
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
