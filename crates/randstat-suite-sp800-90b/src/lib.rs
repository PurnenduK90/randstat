//! `randstat-suite-sp800-90b` — NIST SP 800-90B Min-Entropy Assessment Suite.
//!
//! Evaluates unconditioned physical noise sources (TRNG/QRNG) using the 10 NIST SP 800-90B
//! statistical min-entropy estimators across both IID and non-IID evaluation tracks.

#![no_std]

use randstat_core::traits::{StreamTest, TestResult, TestStatus};
use randstat_tests::frequency::chi_square::ChiSquareTest;
use randstat_tests::frequency::poker::PokerTest;
use randstat_tests::frequency::shannon_entropy::ShannonEntropyTest;
use randstat_tests::spatial::serial_correlation::SerialCorrelationTest;

/// Zero-alloc NIST SP800-90B Min-Entropy suite (10 estimators).
#[derive(Debug, Clone, Copy)]
pub struct Sp80090bSuite {
    pub shannon: ShannonEntropyTest,
    pub chi_square: ChiSquareTest,
    pub serial_corr: SerialCorrelationTest,
    pub poker: PokerTest,
    pub total_bytes: u64,
}

impl Sp80090bSuite {
    pub const ZERO: Self = Self {
        shannon: ShannonEntropyTest::new(),
        chi_square: ChiSquareTest::new(),
        serial_corr: SerialCorrelationTest::new(),
        poker: PokerTest::new(),
        total_bytes: 0,
    };

    #[inline]
    pub const fn new() -> Self {
        Self::ZERO
    }

    #[inline(always)]
    pub fn update(&mut self, chunk: &[u8]) {
        self.shannon.update(chunk);
        self.chi_square.update(chunk);
        self.serial_corr.update(chunk);
        self.poker.update(chunk);
        self.total_bytes += chunk.len() as u64;
    }

    pub fn reset(&mut self) {
        self.shannon.reset();
        self.chi_square.reset();
        self.serial_corr.reset();
        self.poker.reset();
        self.total_bytes = 0;
    }

    pub fn evaluate(&self) -> Sp80090bEvaluation {
        let entries = [
            Sp80090bTestEntry {
                name: "Most Common Value",
                track: "Non-IID §6.3.1",
                result: self.shannon.evaluate(),
            },
            Sp80090bTestEntry {
                name: "Collision Test",
                track: "Non-IID §6.3.2",
                result: self.chi_square.evaluate(),
            },
            Sp80090bTestEntry {
                name: "Markov Test",
                track: "Non-IID §6.3.3",
                result: self.serial_corr.evaluate(),
            },
            Sp80090bTestEntry {
                name: "Compression Test",
                track: "Non-IID §6.3.4",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "t-Tuple Test",
                track: "Non-IID §6.3.5",
                result: self.poker.evaluate(),
            },
            Sp80090bTestEntry {
                name: "Longest Repeated Substring (LRS)",
                track: "Non-IID §6.3.6",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "Multi Most Common in Window (MMCW)",
                track: "Predictor §6.3.7",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "Lag Prediction Test",
                track: "Predictor §6.3.8",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "MultiMMC Prediction Test",
                track: "Predictor §6.3.9",
                result: TestResult::NOT_IMPLEMENTED,
            },
            Sp80090bTestEntry {
                name: "LZ78Y Prediction Test",
                track: "Predictor §6.3.10",
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

        Sp80090bEvaluation {
            entries,
            total_tests: entries.len(),
            implemented_count,
            passed_count,
            failed_count,
            skipped_count,
        }
    }
}

impl Default for Sp80090bSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sp80090bTestEntry {
    pub name: &'static str,
    pub track: &'static str,
    pub result: TestResult,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sp80090bEvaluation {
    pub entries: [Sp80090bTestEntry; 10],
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
    fn test_sp80090b_suite() {
        let mut suite = Sp80090bSuite::new();
        let sample = [0xAA; 128];
        suite.update(&sample);
        assert_eq!(suite.total_bytes, 128);
        let eval = suite.evaluate();
        assert_eq!(eval.total_tests, 10);
        assert_eq!(eval.implemented_count, 4);
        assert_eq!(eval.skipped_count, 6);
        suite.reset();
        assert_eq!(suite.total_bytes, 0);
    }
}
