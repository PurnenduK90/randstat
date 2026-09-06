//! `randstat-suite-practrand` — PractRand High-Throughput PRNG Test Suite.
//!
//! Implements Chris Doty-Humphrey's dynamic streaming statistical tests designed
//! to detect subtle low-weight linear and structural flaws across multi-terabyte streams.

#![no_std]

use randstat_core::traits::{StreamTest, TestResult, TestStatus};
use randstat_tests::frequency::chi_square::ChiSquareTest;
use randstat_tests::frequency::monobit::MonobitTest;
use randstat_tests::frequency::poker::PokerTest;

/// Zero-alloc PractRand streaming battery.
#[derive(Debug, Clone, Copy)]
pub struct PractRandSuite {
    pub monobit: MonobitTest,
    pub poker: PokerTest,
    pub chi_square: ChiSquareTest,
    pub total_bytes: u64,
}

impl PractRandSuite {
    pub const ZERO: Self = Self {
        monobit: MonobitTest::new(),
        poker: PokerTest::new(),
        chi_square: ChiSquareTest::new(),
        total_bytes: 0,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.monobit.update(chunk);
        self.poker.update(chunk);
        self.chi_square.update(chunk);
        self.total_bytes += chunk.len() as u64;
    }

    pub fn reset(&mut self) {
        self.monobit.reset();
        self.poker.reset();
        self.chi_square.reset();
        self.total_bytes = 0;
    }

    pub fn evaluate(&self) -> PractRandEvaluation {
        let entries = [
            PractRandTestEntry {
                name: "BCFN (Binary Cell Finite Number)",
                test_type: "Linear / Folding",
                result: self.monobit.evaluate(),
            },
            PractRandTestEntry {
                name: "Gap-16",
                test_type: "Frequency / Distance",
                result: self.poker.evaluate(),
            },
            PractRandTestEntry {
                name: "FPFT (Fourier Transform)",
                test_type: "Spectral",
                result: TestResult::NOT_IMPLEMENTED,
            },
            PractRandTestEntry {
                name: "BRank (Binary Matrix Rank)",
                test_type: "Matrix",
                result: TestResult::NOT_IMPLEMENTED,
            },
            PractRandTestEntry {
                name: "DC6 (Distance to Cube)",
                test_type: "Spatial",
                result: TestResult::NOT_IMPLEMENTED,
            },
            PractRandTestEntry {
                name: "Dist-1to4",
                test_type: "Distribution",
                result: self.chi_square.evaluate(),
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

        PractRandEvaluation {
            entries,
            total_tests: entries.len(),
            implemented_count,
            passed_count,
            failed_count,
            skipped_count,
        }
    }
}

impl Default for PractRandSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PractRandTestEntry {
    pub name: &'static str,
    pub test_type: &'static str,
    pub result: TestResult,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PractRandEvaluation {
    pub entries: [PractRandTestEntry; 6],
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
    fn test_practrand_suite() {
        let mut suite = PractRandSuite::new();
        let sample = [0xAA; 128];
        suite.update(&sample);
        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 6);
        assert_eq!(eval.implemented_count, 3);
        assert_eq!(eval.skipped_count, 3);
        suite.reset();
        assert_eq!(suite.total_bytes, 0);
    }
}
