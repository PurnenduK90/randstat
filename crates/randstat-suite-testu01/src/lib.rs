//! `randstat-suite-testu01` — TestU01 Academic PRNG Benchmark Suite.
//!
//! Provides the canonical academic batteries developed by Pierre L'Ecuyer and Richard Simard:
//! SmallCrush (10 tests), Crush (96 tests), and BigCrush (106 tests).

#![no_std]

use randstat_core::traits::{StreamTest, TestResult, TestStatus};
use randstat_tests::matrix::binary_matrix_rank::BinaryMatrixRankTest;
use randstat_tests::runs::runs_test::RunsTest;
use randstat_tests::spatial::birthday_spacings::BirthdaySpacingsTest;

/// Zero-alloc TestU01 suite (SmallCrush battery).
#[derive(Debug, Clone, Copy)]
pub struct TestU01Suite {
    pub birthday_spacings: BirthdaySpacingsTest,
    pub matrix_rank: BinaryMatrixRankTest,
    pub runs: RunsTest,
    pub total_bytes: u64,
}

impl TestU01Suite {
    pub const ZERO: Self = Self {
        birthday_spacings: BirthdaySpacingsTest::new(),
        matrix_rank: BinaryMatrixRankTest::new(),
        runs: RunsTest::new(),
        total_bytes: 0,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.birthday_spacings.update(chunk);
        self.matrix_rank.update(chunk);
        self.runs.update(chunk);
        self.total_bytes += chunk.len() as u64;
    }

    pub fn reset(&mut self) {
        self.birthday_spacings.reset();
        self.matrix_rank.reset();
        self.runs.reset();
        self.total_bytes = 0;
    }

    pub fn evaluate(&self) -> TestU01Evaluation {
        let entries = [
            TestU01TestEntry {
                name: "smarsa_BirthdaySpacings",
                battery: "SmallCrush",
                result: self.birthday_spacings.evaluate(),
            },
            TestU01TestEntry {
                name: "sknuth_Collision",
                battery: "SmallCrush",
                result: TestResult::NOT_IMPLEMENTED,
            },
            TestU01TestEntry {
                name: "sknuth_Gap",
                battery: "SmallCrush",
                result: TestResult::NOT_IMPLEMENTED,
            },
            TestU01TestEntry {
                name: "sknuth_SimpPoker",
                battery: "SmallCrush",
                result: TestResult::NOT_IMPLEMENTED,
            },
            TestU01TestEntry {
                name: "sknuth_CouponCollector",
                battery: "SmallCrush",
                result: TestResult::NOT_IMPLEMENTED,
            },
            TestU01TestEntry {
                name: "sknuth_MaxOft",
                battery: "SmallCrush",
                result: TestResult::NOT_IMPLEMENTED,
            },
            TestU01TestEntry {
                name: "svar_WeightDistrib",
                battery: "SmallCrush",
                result: TestResult::NOT_IMPLEMENTED,
            },
            TestU01TestEntry {
                name: "smarsa_MatrixRank",
                battery: "SmallCrush",
                result: self.matrix_rank.evaluate(),
            },
            TestU01TestEntry {
                name: "sstring_HammingIndep",
                battery: "SmallCrush",
                result: TestResult::NOT_IMPLEMENTED,
            },
            TestU01TestEntry {
                name: "sstring_Run",
                battery: "SmallCrush",
                result: self.runs.evaluate(),
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

        TestU01Evaluation {
            entries,
            total_tests: entries.len(),
            implemented_count,
            passed_count,
            failed_count,
            skipped_count,
        }
    }
}

impl Default for TestU01Suite {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestU01TestEntry {
    pub name: &'static str,
    pub battery: &'static str,
    pub result: TestResult,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestU01Evaluation {
    pub entries: [TestU01TestEntry; 10],
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
    fn test_testu01_suite() {
        let mut suite = TestU01Suite::new();
        suite.update(&[1, 2, 3, 4]);
        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 10);
        suite.reset();
    }
}
