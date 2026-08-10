//! Dieharder: Count the 1s in a Stream Test (stub).
//!
//! Counts the number of 1-bits in each byte of a stream. The byte value is
//! mapped to a letter (A–E) based on its popcount (0→A, 1→B, 2→B, 3→C, …, 8→E).
//! The resulting letter stream is tested for five-letter word frequencies.
//!
//! Detects bias in individual bit positions within bytes — a failure indicates
//! some bits are significantly more likely to be 0 or 1 than others.
//!
//! **Status: Stub** — accumulates byte popcount histogram; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Count the 1s in a Stream accumulator (stub).
///
/// Tracks a 9-bucket histogram of byte popcount (0–8 ones per byte).
#[derive(Debug, Clone, Copy)]
pub struct CountOnesStreamTest {
    pub popcount_hist: [u64; 9], // hist[k] = number of bytes with exactly k one-bits
    pub total_bytes: u64,
}

impl Default for CountOnesStreamTest {
    fn default() -> Self {
        Self::new()
    }
}

impl CountOnesStreamTest {
    pub const fn new() -> Self {
        Self {
            popcount_hist: [0u64; 9],
            total_bytes: 0,
        }
    }
}

impl StreamTest for CountOnesStreamTest {
    fn update(&mut self, chunk: &[u8]) {
        for &b in chunk {
            self.popcount_hist[b.count_ones() as usize] += 1;
            self.total_bytes += 1;
        }
    }
    fn reset(&mut self) {
        self.popcount_hist = [0u64; 9];
        self.total_bytes = 0;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: map bytes to A–E via popcount; accumulate overlapping 5-letter words;
        //       compare 5-word frequency table to expected multinomial distribution.
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
