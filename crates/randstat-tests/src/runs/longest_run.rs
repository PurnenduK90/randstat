//! NIST SP800-22 §2.4 — Longest Run of Ones in a Block Test.
//!
//! Tests whether the distribution of the longest runs of ones across 128-bit blocks
//! is consistent with a random sequence.

use randstat_core::algorithms::longest_run::nist_longest_run_m128;
use randstat_core::traits::{StreamTest, TestResult};

/// Longest Run of Ones streaming accumulator (M=128 block size).
#[derive(Debug, Clone, Copy)]
pub struct LongestRunTest {
    pub bin_counts: [u64; 6],
    pub num_blocks: u64,
    pub total_bits: u64,
    pub current_block_bits: u16,
    pub current_run: u16,
    pub max_run_in_block: u16,
}

impl LongestRunTest {
    pub const ZERO: Self = Self {
        bin_counts: [0; 6],
        num_blocks: 0,
        total_bits: 0,
        current_block_bits: 0,
        current_run: 0,
        max_run_in_block: 0,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }
}

impl Default for LongestRunTest {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamTest for LongestRunTest {
    fn update(&mut self, chunk: &[u8]) {
        for &byte in chunk {
            self.total_bits += 8;

            for shift in (0..8).rev() {
                let bit = (byte >> shift) & 1;
                if bit == 1 {
                    self.current_run += 1;
                    if self.current_run > self.max_run_in_block {
                        self.max_run_in_block = self.current_run;
                    }
                } else {
                    self.current_run = 0;
                }

                self.current_block_bits += 1;
                if self.current_block_bits == 128 {
                    let bin = match self.max_run_in_block {
                        0..=4 => 0,
                        5 => 1,
                        6 => 2,
                        7 => 3,
                        8 => 4,
                        _ => 5,
                    };
                    self.bin_counts[bin] += 1;
                    self.num_blocks += 1;
                    self.current_block_bits = 0;
                    self.current_run = 0;
                    self.max_run_in_block = 0;
                }
            }
        }
    }

    fn reset(&mut self) {
        self.bin_counts = [0; 6];
        self.num_blocks = 0;
        self.total_bits = 0;
        self.current_block_bits = 0;
        self.current_run = 0;
        self.max_run_in_block = 0;
    }

    fn evaluate(&self) -> TestResult {
        nist_longest_run_m128(&self.bin_counts, self.num_blocks)
    }
}
