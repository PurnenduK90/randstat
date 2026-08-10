//! Dieharder: Runs Up and Down Test (stub).
//!
//! Counts the number of "runs" of consecutively ascending or descending values
//! in the sequence (treating the byte stream as a sequence of integers). The
//! distribution of run lengths should follow a known distribution for random data.
//!
//! Note: this is **different** from the NIST Runs Test (§2.3), which counts runs
//! of bits above or below the median. This test looks at monotone subsequences
//! in the numeric sequence of byte values.
//!
//! **Status: Stub** — accumulates total bytes; returns dummy p-value of 0.5.

use randstat_core::traits::{StreamTest, TestResult};

/// Runs Up and Down Test accumulator (stub).
#[derive(Debug, Clone, Copy, Default)]
pub struct RunsUpDownTest {
    pub total_bytes: u64,
    pub last_byte: u8,
    pub has_prev: bool,
}

impl RunsUpDownTest {
    pub const fn new() -> Self {
        Self {
            total_bytes: 0,
            last_byte: 0,
            has_prev: false,
        }
    }
}

impl StreamTest for RunsUpDownTest {
    fn update(&mut self, chunk: &[u8]) {
        self.total_bytes += chunk.len() as u64;
        if let Some(&last) = chunk.last() {
            self.last_byte = last;
            self.has_prev = true;
        }
    }
    fn reset(&mut self) {
        self.total_bytes = 0;
        self.last_byte = 0;
        self.has_prev = false;
    }
    fn evaluate(&self) -> TestResult {
        // TODO: track sign changes (+/-) in the byte-to-byte differences;
        //       measure run lengths; compare run-length frequency distribution
        //       to theoretical values using chi-square.
        TestResult {
            statistic: 0.0,
            p_value: 0.5,
            passed: true,
        }
    }
}
