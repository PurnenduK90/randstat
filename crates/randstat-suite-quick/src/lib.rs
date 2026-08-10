//! `randstat-suite-quick` — Fast 4-test screening suite.
//!
//! A curated set of statistically powerful tests that run in microseconds
//! even on multi-GB streams. Ideal for CI pre-flight checks and live dashboards.
//!
//! Tests:
//! - [`MonobitTest`]       — NIST SP800-22 §2.1 bit-level balance
//! - [`ShannonEntropyTest`] — Information density (bits/byte)
//! - [`ChiSquareTest`]    — Byte frequency uniformity (df=255)
//! - [`ArithmeticMeanTest`] — Byte mean (ideal: 127.5)

#![no_std]

use randstat_core::traits::StreamTest;
use randstat_tests::frequency::arithmetic_mean::ArithmeticMeanTest;
use randstat_tests::frequency::chi_square::ChiSquareTest;
use randstat_tests::frequency::monobit::MonobitTest;
use randstat_tests::frequency::shannon_entropy::ShannonEntropyTest;

/// Zero-alloc fast screening suite.
#[derive(Debug, Clone, Copy)]
pub struct QuickSuite {
    pub monobit: MonobitTest,
    pub shannon: ShannonEntropyTest,
    pub chi_square: ChiSquareTest,
    pub arithmetic_mean: ArithmeticMeanTest,
}

impl QuickSuite {
    pub const ZERO: Self = Self {
        monobit: MonobitTest::new(),
        shannon: ShannonEntropyTest::new(),
        chi_square: ChiSquareTest::new(),
        arithmetic_mean: ArithmeticMeanTest::new(),
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.monobit.update(chunk);
        self.shannon.update(chunk);
        self.chi_square.update(chunk);
        self.arithmetic_mean.update(chunk);
    }

    pub fn reset(&mut self) {
        self.monobit.reset();
        self.shannon.reset();
        self.chi_square.reset();
        self.arithmetic_mean.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_suite() {
        let mut suite = QuickSuite::new();
        suite.update(&[1, 2, 3, 4, 5]);
        assert_eq!(suite.monobit.total_bits, 40);

        let mut default_suite = QuickSuite::default();
        default_suite.update(&[1, 2, 3]);
        default_suite.reset();
        assert_eq!(default_suite.monobit.total_bits, 0);
    }
}

impl Default for QuickSuite {
    fn default() -> Self {
        Self::new()
    }
}
