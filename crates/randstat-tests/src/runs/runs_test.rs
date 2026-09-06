//! NIST SP800-22 §2.3 — Runs Test.
//!
//! Tests whether the number of runs of ones and zeros of various lengths
//! is consistent with a random sequence.

use randstat_core::algorithms::runs::nist_runs;
use randstat_core::traits::{StreamTest, TestResult};

/// Runs test streaming accumulator.
#[derive(Debug, Clone, Copy)]
pub struct RunsTest {
    pub ones: u64,
    pub total_bits: u64,
    pub transitions: u64,
    pub last_bit: i8,
}

impl RunsTest {
    pub const ZERO: Self = Self {
        ones: 0,
        total_bits: 0,
        transitions: 0,
        last_bit: -1,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }
}

impl Default for RunsTest {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamTest for RunsTest {
    fn update(&mut self, chunk: &[u8]) {
        for &byte in chunk {
            self.ones += byte.count_ones() as u64;
            self.total_bits += 8;

            for shift in (0..8).rev() {
                let bit = ((byte >> shift) & 1) as i8;
                if self.last_bit != -1 && bit != self.last_bit {
                    self.transitions += 1;
                }
                self.last_bit = bit;
            }
        }
    }

    fn reset(&mut self) {
        self.ones = 0;
        self.total_bits = 0;
        self.transitions = 0;
        self.last_bit = -1;
    }

    fn evaluate(&self) -> TestResult {
        nist_runs(self.ones, self.total_bits, self.transitions)
    }
}
