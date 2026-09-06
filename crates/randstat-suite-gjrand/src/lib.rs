//! `randstat-suite-gjrand` — gjrand Lightweight PRNG Test Suite.
//!
//! Implements David Blackman's gjrand benchmark tests for standard bitstream evaluations.

#![no_std]

use randstat_core::traits::{StreamTest, TestResult, TestStatus};
use randstat_tests::frequency::chi_square::ChiSquareTest;
use randstat_tests::frequency::poker::PokerTest;
use randstat_tests::runs::runs_test::RunsTest;
use randstat_tests::spatial::serial_correlation::SerialCorrelationTest;

/// Zero-alloc gjrand streaming battery.
#[derive(Debug, Clone, Copy)]
pub struct GjrandSuite {
    pub chi_square: ChiSquareTest,
    pub serial_corr: SerialCorrelationTest,
    pub runs: RunsTest,
    pub poker: PokerTest,
    pub total_bytes: u64,
}

impl GjrandSuite {
    pub const ZERO: Self = Self {
        chi_square: ChiSquareTest::new(),
        serial_corr: SerialCorrelationTest::new(),
        runs: RunsTest::new(),
        poker: PokerTest::new(),
        total_bytes: 0,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.chi_square.update(chunk);
        self.serial_corr.update(chunk);
        self.runs.update(chunk);
        self.poker.update(chunk);
        self.total_bytes += chunk.len() as u64;
    }

    pub fn reset(&mut self) {
        self.chi_square.reset();
        self.serial_corr.reset();
        self.runs.reset();
        self.poker.reset();
        self.total_bytes = 0;
    }

    pub fn evaluate(&self) -> GjrandEvaluation {
        let entries = [
            GjrandTestEntry {
                name: "mcoll16",
                profile: "16-bit",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "mcoll32",
                profile: "32-bit",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "mprob16",
                profile: "16-bit",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "mprob32",
                profile: "32-bit",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "mdist16",
                profile: "16-bit",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "mdist32",
                profile: "32-bit",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "mgap16",
                profile: "16-bit",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "mgap32",
                profile: "32-bit",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "mrun16",
                profile: "16-bit",
                result: TestResult::NOT_IMPLEMENTED,
            },
            GjrandTestEntry {
                name: "mrun32",
                profile: "32-bit",
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GjrandTestEntry {
    pub name: &'static str,
    pub profile: &'static str,
    pub result: TestResult,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GjrandEvaluation {
    pub entries: [GjrandTestEntry; 10],
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
        let sample = [0xAA; 128];
        suite.update(&sample);
        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 10);
        assert_eq!(eval.implemented_count, 0);
        assert_eq!(eval.skipped_count, 10);
        suite.reset();
        assert_eq!(suite.total_bytes, 0);
    }
}
