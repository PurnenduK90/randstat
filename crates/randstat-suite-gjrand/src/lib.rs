//! `randstat-suite-gjrand` — gjrand Lightweight PRNG Test Suite.
//!
//! Implements David Blackman's gjrand benchmark tests for standard bitstream evaluations.

#![no_std]

use randstat_core::traits::{StreamTest, TestResult, TestStatus};
use randstat_tests::frequency::chi_square::ChiSquareTest;

/// Zero-alloc gjrand streaming battery.
#[derive(Debug, Clone, Copy)]
pub struct GjrandSuite {
    pub chi_square: ChiSquareTest,
    pub total_bytes: u64,
}

impl GjrandSuite {
    pub const ZERO: Self = Self {
        chi_square: ChiSquareTest::new(),
        total_bytes: 0,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.chi_square.update(chunk);
        self.total_bytes += chunk.len() as u64;
    }

    pub fn reset(&mut self) {
        self.chi_square.reset();
        self.total_bytes = 0;
    }

    pub fn evaluate(&self) -> GjrandEvaluation {
        let entries = [
            GjrandTestEntry {
                name: "Uniformity (Chi-Square)",
                profile: "Standard",
                result: self.chi_square.evaluate(),
            },
            GjrandTestEntry {
                name: "Word Correlation",
                profile: "Standard",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "Run Structure",
                profile: "Standard",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "Poker Variations",
                profile: "Standard",
                result: TestResult::NOT_IMPLEMENTED,
            },
        ];

        let mut implemented_count = 0;
        let mut passed_count = 0;
        let mut failed_count = 0;
        let mut skipped_count = 0;

        for entry in &entries {
            match entry.result.status {
                TestStatus::Passed => {
                    implemented_count += 1;
                    passed_count += 1;
                }
                TestStatus::Failed => {
                    implemented_count += 1;
                    failed_count += 1;
                }
                TestStatus::NotImplemented | TestStatus::InsufficientData => {
                    skipped_count += 1;
                }
            }
        }

        GjrandEvaluation {
            entries,
            total_tests: entries.len(),
            implemented_count,
            passed_count,
            failed_count,
            skipped_count,
        }
    }
}

impl Default for GjrandSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GjrandTestEntry {
    pub name: &'static str,
    pub profile: &'static str,
    pub result: TestResult,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GjrandEvaluation {
    pub entries: [GjrandTestEntry; 4],
    pub total_tests: usize,
    pub implemented_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub skipped_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gjrand_suite() {
        let mut suite = GjrandSuite::new();
        suite.update(&[1, 2, 3]);
        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 4);
        suite.reset();
    }
}
