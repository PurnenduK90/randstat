//! NIST SP800-22 Ã‚Â§2.1 Ã¢â‚¬â€ Frequency (Monobit) Test.
//!
//! `MonobitTest` is the streaming accumulator. The evaluation formula lives in
//! [`randstat_core::algorithms::monobit::nist_monobit`].

use randstat_core::algorithms::monobit::nist_monobit;
use randstat_core::traits::{StreamTest, TestResult};

/// NIST SP800-22 Ã‚Â§2.1 Frequency (Monobit) streaming accumulator.
#[derive(Debug, Clone, Copy, Default)]
pub struct MonobitTest {
    pub ones: u64,
    pub total_bits: u64,
}

impl MonobitTest {
    pub const fn new() -> Self {
        Self {
            ones: 0,
            total_bits: 0,
        }
    }
}

impl StreamTest for MonobitTest {
    fn update(&mut self, chunk: &[u8]) {
        for &byte in chunk {
            self.ones += byte.count_ones() as u64;
            self.total_bits += 8;
        }
    }

    fn reset(&mut self) {
        self.ones = 0;
        self.total_bits = 0;
    }

    fn evaluate(&self) -> TestResult {
        nist_monobit(self.ones, self.total_bits)
    }
}
