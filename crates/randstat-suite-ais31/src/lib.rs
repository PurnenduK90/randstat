//! `randstat-suite-ais31` — German BSI AIS 20 / AIS 31 Physical TRNG Test Suite.
//!
//! Implements the German Federal Office for Information Security (BSI) evaluation
//! criteria for True Random Number Generators (TRNG Class PTG.2 / PTG.3).

#![no_std]

use randstat_core::traits::{StreamTest, TestResult, TestStatus};
use randstat_tests::frequency::monobit::MonobitTest;
use randstat_tests::frequency::poker::PokerTest;
use randstat_tests::frequency::shannon_entropy::ShannonEntropyTest;
use randstat_tests::runs::longest_run::LongestRunTest;
use randstat_tests::runs::runs_test::RunsTest;
use randstat_tests::spatial::serial_correlation::SerialCorrelationTest;

/// Zero-alloc BSI AIS 20 / AIS 31 Suite (Tests T0–T8).
#[derive(Debug, Clone, Copy)]
pub struct Ais31Suite {
    pub monobit: MonobitTest,
    pub poker: PokerTest,
    pub runs: RunsTest,
    pub long_runs: LongestRunTest,
    pub autocorrelation: SerialCorrelationTest,
    pub shannon: ShannonEntropyTest,
}

impl Ais31Suite {
    pub const ZERO: Self = Self {
        monobit: MonobitTest::new(),
        poker: PokerTest::new(),
        runs: RunsTest::new(),
        long_runs: LongestRunTest::new(),
        autocorrelation: SerialCorrelationTest::new(),
        shannon: ShannonEntropyTest::new(),
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.monobit.update(chunk);
        self.poker.update(chunk);
        self.runs.update(chunk);
        self.long_runs.update(chunk);
        self.autocorrelation.update(chunk);
        self.shannon.update(chunk);
    }

    pub fn reset(&mut self) {
        self.monobit.reset();
        self.poker.reset();
        self.runs.reset();
        self.long_runs.reset();
        self.autocorrelation.reset();
        self.shannon.reset();
    }

    pub fn evaluate(&self) -> Ais31Evaluation {
        let entries = [
            Ais31TestEntry {
                name: "Test T0: Disjointness",
                standard_id: "T0",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Ais31TestEntry {
                name: "Test T1: Monobit",
                standard_id: "T1",
                result: self.monobit.evaluate(),
            },
            Ais31TestEntry {
                name: "Test T2: Poker Test",
                standard_id: "T2",
                result: self.poker.evaluate(),
            },
            Ais31TestEntry {
                name: "Test T3: Runs",
                standard_id: "T3",
                result: self.runs.evaluate(),
            },
            Ais31TestEntry {
                name: "Test T4: Long Runs",
                standard_id: "T4",
                result: self.long_runs.evaluate(),
            },
            Ais31TestEntry {
                name: "Test T5: Autocorrelation",
                standard_id: "T5",
                result: self.autocorrelation.evaluate(),
            },
            Ais31TestEntry {
                name: "Test T6: Uniform Distribution",
                standard_id: "T6",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Ais31TestEntry {
                name: "Test T7: Comparative Test",
                standard_id: "T7",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Ais31TestEntry {
                name: "Test T8: Shannon Entropy",
                standard_id: "T8",
                result: self.shannon.evaluate(),
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

        Ais31Evaluation {
            entries,
            total_tests: entries.len(),
            implemented_count,
            passed_count,
            failed_count,
            skipped_count,
        }
    }
}

impl Default for Ais31Suite {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ais31TestEntry {
    pub name: &'static str,
    pub standard_id: &'static str,
    pub result: TestResult,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ais31Evaluation {
    pub entries: [Ais31TestEntry; 9],
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
    fn test_ais31_suite() {
        let mut suite = Ais31Suite::new();
        suite.update(&[1, 2, 3, 4, 5]);
        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 9);
        assert!(eval.implemented_count >= 1);
        suite.reset();
    }
}
